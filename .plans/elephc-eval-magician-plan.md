# Piano: eval, elephc-magician e literal eval AOT

## Task

- [x] Definire la semantica target di `eval`: scope caller visibile, scritture
  persistenti, variabili create visibili dopo eval, `unset`, output, parse
  error, `return` locale al frammento, dichiarazioni dinamiche e `$this`.
- [x] Aggiungere `crates/elephc-magician` come bridge opzionale e linkarlo solo
  quando il programma richiede il fallback eval runtime.
- [x] Aggiungere ABI, `RuntimeFeatures`, linker bridge e runtime helper per
  chiamare `__elephc_eval_execute` dal backend EIR corrente.
- [x] Implementare `ElephcEvalContext` e `ElephcEvalScope` condivisi tra codice
  nativo e interpreter, inclusi flush/reload dei locals osservabili.
- [x] Implementare parser runtime, EvalIR/interpreter e value bridge per il
  subset eval supportato da magician.
- [x] Supportare nel fallback magician variabili, assegnamenti, output, return,
  control flow, arrays, include/require, chiamate dinamiche, dichiarazioni,
  classi/oggetti, reflection, callable, references/by-ref e cleanup errori per
  il subset coperto dai test.
- [x] Modellare `eval` come barriera di effetti per optimizer/type checker:
  niente DCE, niente propagazione costanti attraverso locals osservabili e
  fallback dinamico dove necessario.
- [x] Aggiungere benchmark magician ripetibili con varianti Elephc native,
  Elephc eval, PHP native e PHP eval.
- [x] Aggiungere parse cache, parse-error cache e include parse/file cache senza
  congelare context, scope, magic constants o include_once state.
- [x] Aggiungere cache per lookup simboli eval, direct builtin dispatch,
  callable resolution cache e ottimizzazioni conservative su `RuntimeValueOps`.
- [x] Aggiungere fast path unboxed scalar, linear EvalIR/stack VM opzionale e
  ottimizzazioni mirate per array/reference/COW nel bridge.
- [x] Implementare literal `eval` AOT conservativo per scalari, output, return,
  store/scope read-write e marker assembly di AOT/fallback.
- [x] Estendere literal eval AOT a locals interni, `while`, `if`, `break`,
  `continue`, confronti/truthiness, modulo e benchmark prime-sum fino a
  `100000`.
- [x] Estendere literal eval AOT a builtins statici comuni, funzioni statiche
  note, metodi statici pubblici tipizzati e callback statici via
  `call_user_func*()`.
- [x] Evitare il link a `elephc_magician` per programmi con soli eval literal
  fully AOT.
- [x] Aggiornare i test parity per distinguere builtin condivisi,
  builtin eval-only documentati e builtin static-only non ancora presenti in
  magician.
- [ ] Ridurre il mini-codegen AOT manuale residuo e convergere verso funzioni
  EIR interne per i frammenti literal supportati.
- [ ] Espandere AOT solo dove semanticamente coperto: arrays/iterables completi,
  object/member access, references/by-ref, `global`, `static`, variable
  variables, `try`/`throw`, include/require e dichiarazioni restano fallback
  finche' non hanno modello e test dedicati.
- [ ] Chiudere o mantenere esplicitamente il gap dei builtin static-only:
  implementarli in magician oppure tenerli in allowlist testata finche' eval non
  li espone.
- [ ] Promuovere i benchmark di accettazione AOT piu' utili nella suite
  benchmark permanente, senza includere compile/link time nei runtime numbers.
- [ ] Aggiornare docs utente/internals dopo ogni estensione semantica del
  subset eval o AOT.
- [ ] Eseguire verifiche focused sui tre target supportati per ogni modifica che
  tocca ABI, runtime ownership, codegen eval o fallback/AOT selection.

## Scope del piano

Questo piano sostituisce e fonde:

- `.plans/elephc-eval-complete-plan.md`
- `.plans/elephc-eval-aot-complete-plan.md`
- `.plans/elephc-magician-performance-plan.md`

Il piano resta in `.plans` per tracciare solo il lavoro eval/magician ancora
aperto. Le sezioni completate documentano lo stato raggiunto e servono come
guardrail per non reintrodurre vecchi approcci o regressioni.

## Stato corrente

Il supporto eval esiste su due percorsi:

1. Fallback runtime tramite `libelephc-magician`, chiamato da
   `__elephc_eval_execute`.
2. Literal eval AOT, quando il frammento e' una stringa nota a compile time e il
   classificatore lo considera semanticamente sicuro.

Il backend attivo dopo il rebase su `main` e' il percorso EIR sotto
`src/ir_lower/`, `src/ir_passes/` e `src/codegen/lower_inst/`. I riferimenti
storici a `src/codegen_ir/` nei vecchi piani sono obsoleti.

I file centrali attuali sono:

- `crates/elephc-magician/src/`
- `src/eval_aot.rs`
- `src/ir_lower/expr/mod.rs`
- `src/ir_lower/program.rs`
- `src/codegen/lower_inst/builtins/eval.rs`
- `src/codegen_support/runtime/eval_bridge.rs`
- `src/codegen_support/runtime_features.rs`
- `tests/codegen/eval.rs`
- `tests/codegen/eval_callables.rs`
- `tests/codegen/eval_callable_ref_errors.rs`
- `tests/codegen/eval_constructors.rs`
- `tests/codegen/eval_closures.rs`
- `tests/codegen/eval_reflection_invocation.rs`
- `tests/builtin_parity_tests.rs`

## Architettura consolidata

### Fallback magician

`elephc-magician` e' una staticlib bridge opzionale. I programmi senza eval
runtime non devono linkarla. Il fallback resta obbligatorio per:

- eval dinamico;
- literal eval non parseabile o non supportato dal classificatore AOT;
- costrutti con semantica runtime non ancora modellata in AOT;
- dichiarazioni dinamiche, include/require, references/by-ref, global/static,
  variable variables, oggetti/members dinamici e throwable finche' non coperti.

Il fallback riceve:

- context globale eval;
- scope locale eval;
- scope globale quando serve;
- puntatore/lunghezza del codice;
- buffer risultato.

Il value model non deve divergere da quello nativo: boxing, refcount, COW,
references e cleanup devono rimanere coerenti con il runtime elephc.

### Scope sync

Il codice nativo deve sincronizzare con lo scope eval solo cio' che e'
osservabile dal frammento:

- prima della chiamata: flush delle variabili lette/scritte quando serve;
- durante eval: magician opera sullo scope condiviso;
- dopo eval: reload delle variabili che possono essere state scritte, create o
  unsettate.

Quando l'analisi non e' precisa, la semantica vince sulla performance: usare il
fallback o trattare il frammento come barriera piu' forte.

### Literal eval AOT

Il compilatore analizza il frammento literal a compile time:

```text
literal string
  -> parse come PHP fragment
  -> normalizzazione/nameresolution compatibile col contesto
  -> classificazione eligibility AOT
  -> piano read/write/call/fallback
  -> lowering nativo o fallback magician
```

Il piano AOT deve preservare:

- `return expr;` ritorna da eval, non dal caller;
- fallthrough senza `return` produce `null`;
- output resta side effect visibile;
- variabili caller note a compile time sono leggibili/scrivibili;
- variabili create dal frammento sono visibili dopo eval se il path AOT lo
  dichiara supportato;
- ogni costrutto non coperto rimane fallback esplicito.

I path AOT emettono marker assembly tipo `eval literal AOT compiled...`; i
fallback emettono marker con ragione leggibile quando possibile.

## Lavoro completato

### Eval runtime e bridge

Completato:

- crate `elephc-magician`;
- ABI C/Rust verso `__elephc_eval_execute`;
- bridge linker `elephc_magician`;
- detection runtime features;
- eval language construct nel checking/lowering;
- materialized scope, context e value bridge;
- flush/reload dei locals osservabili;
- error/status mapping e cleanup.

La copertura codegen e interpreter include eval in top-level, funzioni, metodi,
scope condiviso, nested eval, return/output, variabili create, mutation di
locals, callables, constructors, closures e reflection.

### Interpreter magician

Completato per il subset corrente:

- lexer/parser runtime per fragment eval senza tag `<?php`;
- EvalIR/interpreter;
- expressions/statements di base;
- control flow;
- arrays e COW nel path supportato;
- include/require;
- dynamic functions/classes e metadata runtime;
- builtin registry/dispatch lato interpreter;
- callable forms e `Closure::fromCallable`;
- classi, interfacce, trait, enum, static members e reflection nel subset
  coperto;
- throw/fatal/status handling dove supportato.

### Performance magician

Completato:

- benchmark suite `scripts/benchmark_magician.py` con fixtures in
  `benchmarks/magician/cases/`;
- parse cache e parse-error cache;
- include cache con metadata validation;
- lookup cache per simboli eval/native;
- direct builtin dispatch per hot path;
- callable resolution cache conservativa;
- riduzione di chiamate `RuntimeValueOps` su output/scalari semplici;
- evaluator temporaneo int/bool per assignment/return/condition;
- linear EvalIR opzionale per straight-line fragments;
- fast path stretti per indexed-array writes.

### Literal eval AOT

Completato:

- `EvalLiteralCall` conserva il payload literal in EIR;
- `src/eval_aot.rs` classifica eligibility e fallback reason;
- `src/codegen/lower_inst/builtins/eval.rs` prova AOT prima del bridge;
- supporto per scalari, arithmetic, concat/output, print, return, stores,
  read/write scope e boxed Mixed scope paths;
- supporto per locals interni, assignment/compound, while/if/break/continue,
  modulo, confronti e truthiness sufficienti per il prime benchmark;
- supporto per builtins statici comuni;
- supporto per funzioni statiche note;
- supporto per metodi statici pubblici tipizzati;
- supporto per callback statici in `call_user_func()` e
  `call_user_func_array()`, incluse forme string, array, `Class::class` e
  first-class statici immediati;
- test che verificano assenza di `__elephc_eval_execute` e assenza del link a
  `elephc_magician` per frammenti fully AOT;
- benchmark prime-sum fino a `100000` senza bridge, output `454396537`.

## Lavoro aperto

### 1. Convergenza AOT verso funzioni EIR interne

Il debito principale e' ridurre il mini-codegen manuale in
`src/codegen/lower_inst/builtins/eval.rs`.

Direzione:

- rappresentare ogni frammento AOT come funzione EIR interna con ABI speciale;
- dichiarare locals del frammento separati dai locals caller;
- introdurre primitive EIR o helper builtin per:
  - `eval_scope_get`;
  - `eval_scope_set`;
  - return/fallthrough `null`;
  - status/fatal propagation;
- far passare la funzione AOT da validator, optimizer, regalloc e backend
  target-aware;
- mantenere fallback magician come compat path.

Done criteria:

- nessuna crescita ulteriore del mini-backend manuale per nuovi costrutti;
- test AOT esistenti continuano a passare;
- assembly marker resta esplicito;
- nessuna regressione su macOS ARM64, Linux ARM64, Linux x86_64.

### 2. Estensione AOT oltre il subset statico attuale

Ogni nuovo costrutto deve essere introdotto solo con modello semantico e test.
Priorita' ragionevole:

1. arrays/iterables in AOT quando COW e ownership sono chiari;
2. object/member access staticamente risolvibile;
3. references/by-ref solo se il modello ref-cell e' identico al runtime;
4. `global`, `static`, variable variables;
5. `try`/`throw`;
6. include/require;
7. declarations dentro eval.

Tutto cio' che non e' modellato resta fallback.

### 3. Builtin parity compiler/eval

`tests/builtin_parity_tests.rs` distingue:

- builtin condivisi compiler/eval;
- builtin eval-only documentati;
- builtin static-only registrati nel compiler ma non ancora esposti da
  magician.

Quando un builtin static-only viene implementato in magician:

- rimuoverlo dall'allowlist static-only;
- aggiungere metadata signature eval;
- aggiungere dispatch interpreter;
- aggiungere test named/positional se rilevante;
- aggiornare benchmark solo se il builtin entra in hot path eval.

### 4. Benchmark e misurazione

La suite benchmark esiste. Il lavoro aperto e':

- decidere quali benchmark AOT devono essere permanenti;
- evitare sempre compile/assemble/link time nei runtime numbers;
- mantenere almeno un caso prime-loop e un caso algebra-heavy come regressione
  manuale o CI artifact;
- conservare output correctness contro PHP dove pratico.

### 5. Documentazione

Aggiornare docs quando cambia il subset:

- eval abilita runtime dinamico opzionale;
- literal eval AOT non embedda parser/compiler nel binario;
- fallback magician resta semantica di compatibilita';
- programmi fully AOT non linkano `elephc_magician`;
- costrutti ancora fallback devono essere documentati se user-visible.

## Test e verifiche

Per modifiche strette al planner/lowering AOT:

```bash
cargo check
cargo test --test codegen_tests literal_eval_static
cargo test --test codegen_tests test_literal_eval_prime_loop_uses_aot_without_execute_bridge
git diff --check
```

Per modifiche a bridge runtime o interpreter:

```bash
cargo check
cargo test -p elephc-magician <filter>
cargo test --test codegen_tests eval_<filter>
git diff --check
```

Per modifiche a ABI/codegen/runtime ownership:

```bash
cargo check
cargo test --test codegen_tests <focused_eval_filter>
./scripts/test-linux-x86_64.sh <focused_eval_filter>
./scripts/test-linux-arm64.sh <focused_eval_filter>
git diff --check
```

Per benchmark manuali:

```bash
python3 scripts/benchmark_magician.py --case algebra_heavy --iterations 5 --warmup 1
python3 scripts/benchmark_magician.py --case literal_scalar_aot --iterations 5 --warmup 1
```

## Rischi

- Scope sync incompleta puo' creare variabili stale o mancare creazioni/unset.
- Duplicare codegen AOT manuale crea un secondo backend difficile da mantenere.
- Treating eval as ordinary static code can break PHP eval semantics.
- References, COW, arrays and object properties can introduce double-free,
  leaks or missed mutations if bypassano helper runtime.
- `eval('$x + 1;')` ritorna `null`, non l'ultima espressione.
- Fallback selection troppo aggressiva puo' miscompilare codice dinamico.
- Ottimizzazioni magician non devono congelare context/scope/magic constants.
- Ogni path nuovo deve restare target-aware su macOS ARM64, Linux ARM64 e
  Linux x86_64.

## Criteri di completamento finale

Il lavoro eval/magician puo' considerarsi chiuso quando:

1. il fallback magician copre in modo testato il subset PHP dichiarato;
2. ogni literal eval supportato usa AOT o fallback esplicito con reason;
3. il subset AOT non dipende da un mini-backend manuale non manutenibile;
4. programmi fully AOT non linkano `elephc_magician`;
5. static/eval builtin parity non ha allowlist stale;
6. benchmark prime-loop e algebra-heavy restano corretti e misurabili;
7. i tre target supportati hanno copertura focused per ogni cambio ABI/codegen;
8. docs e test riflettono esattamente il subset supportato e i fallback.
