# Budget coopératif du candidat δ — S230, 2026-09-14

## Contrat et critère de réception déclarés avant construction

Lot J2/S200-1/A244, application de SPEC-004 §4.1 et ADR-007/012, sans modifier I-05.
`step_budgeted` reçoit un budget en millisecondes et une horloge monotone injectée. Il publie
soit un pas entier avec son rapport numérique, soit **zéro temps avancé et tout dt restant**.
Un arrêt faute de temps conserve u/w/p bit à bit ; le prochain appel recommence le pas depuis
cet état. Les calculs interrompus ne sont ni sérialisés ni conservés comme progression acquise.

Contrôles dans préparation, advection, second membre, pression, correction, diagnostics et
validation. Tranches d'au plus 64 éléments entre contrôles pour les parcours ; une réduction
d'hôte porte au plus 64 cellules, avec les mêmes groupes et le même ordre de fusion qu'avant.
Le retour arrière échange des buffers préalloués en temps constant ; aucune recopie de domaine
après expiration. Les buffers sortis temporairement sont rendus même lors d'un refus.

Le pas à plafond d'itérations et son enveloppe de mesure restent disponibles pour les bancs.
Le chemin non limité doit garder ses bits et ses tests. La pression reste expérimentale f64 ;
aucune exception à I-08 pour δ ni admission B3 n'est déduite de ce lot.

**Garantie coopérative, pas préemption.** L'hôte doit fournir une horloge non bloquante et des
réductions à durée bornée. L'expiration est observée au prochain contrôle ; le dépassement peut
comprendre une tranche, un appel d'hôte, le contrôle et le retour. Une désallocation massive ou
un rollback linéaire après expiration seraient des défauts, pas des marges admissibles.
Le maximum local observé n'est pas une borne de pire cas système : I-05 complet reste à recevoir
par admission des blocs et marges sur le matériel/hôte visés. Une horloge reculant est refusée.

Réception prévue : expiration à chaque phase et après plusieurs itérations, état non nul
intact puis continuation identique à un témoin ; zéro budget, horloge reculant, non-fini et
absence d'allocation avec témoin positif. Mesurer chemin complet, coût des contrôles et retard
observé au retour sur grilles/charges déclarées, sans ajuster le seuil après mesure.

## Réception construite

`delta_projection::Volume::step_budgeted(dt, max_iters, budget_ms, jobs, clock)` retourne
`BudgetReport` : `advanced_dt`, `remaining_dt`, `elapsed_ns`, `stopped_at`, rapport numérique
optionnel. Expiration : zéro avancé, dt entier restant, phase indiquée, aucun rapport numérique
inventé. Succès : dt entier avancé, rapport reçu, y compris sa dégradation au plafond de pression.
Le temps rapporté est celui du **dernier contrôle**, le retour O(1) est mesuré par l'hôte.
Une mesure d'abandon ne remplace pas le coût d'un bloc avancé dans `Caps` (valeur inconnue).

Les anciennes sauvegardes S200 deviennent une paire de buffers de publication/travail : copie
interruptible avant échange, puis annulation par trois échanges de Vec, sans copie ni libération
proportionnelle au domaine. Mémoire allouée inchangée. Les `mem::take` de rhs/tmp sont rendus
avant toute propagation de refus. Une boucle de colonnes est aussi contrôlée quand elle ne
contient aucune face horizontale intérieure (nz=1).

- **204 points d'expiration** sur un domaine 8×4 non au repos : toutes les lectures d'horloge
  après l'origine sont tour à tour le point d'expiration. Huit phases atteintes, u/w/p identiques
  bit à bit, prochain pas identique au témoin. Les restes de travail internes ne contaminent pas
  la reprise. Témoin d'allocation positif puis aucun appel au tas dans abandon et reprise.
- Budget nul, budget négatif/NaN/infini/hors u64, horloge reculant, erreur numérique pendant le
  calcul : résultats explicites et champs conservés. Un arrêt de processus/panic d'hôte n'est
  pas reçu par ces tests.
- Domaine 32×16, plafonds 500/1/0/500 : chaque réduction d'hôte limitée à 64 cellules ; les
  rapports et u/w/p retrouvent le chemin illimité. Les petits plafonds publient leur dégradation.
- Sept tests d'exécution δ et huit tests unitaires passent. Filtre S199 rejoué : empreinte
  **0x0ad3f695685ca27a inchangée**, ordre 1,947 plat, 0,898 lisse, 0,895 marche. Le défaut spatial
  n'est ni corrigé ni masqué par le budget ; candidat toujours non admissible B3.

## Coût et retards observés

Machine locale Windows x86_64, AMD Ryzen AI7 350, Rust 1.97 ; CPU séquentiel, aucun GPU.
Exemple `delta_budget_cost` : domaine 8×4 m, fond plat 0,4 m, rho=1025, g=9,81,
dt=1/60 s, surface z0+0,02 sin(2π(i+0,5)/nx), plafond512. Trois échauffements puis101 mesures,
vitesses remises à zéro avant chaque pas, pression antérieure préservée en cas d'abandon.
Temps externe du pas **retour compris**, configuration/injection/assertions/I/O hors fenêtre.
Mêmes résultats complets que le témoin vérifiés, et état inchangé à chaque abandon.

Techniques présentes : contrôles64, petits appels de réduction, buffers préalloués/rollbackO(1).
Absentes : préemption, marge de pire cas, admission/ordonnanceur, pressionf32, surface mobile,
3D, GPU. Ce banc n'inclut pas B/W/rendu ; les2ms ne sont pas attribués à chaque bloc.

Deux passages : le second après ajout du contrôle de colonne vide, sans changement numérique.
Le premier est conservé ci-dessous pour ne pas effacer un pic simplement parce qu'il n'a pas
été revu. Tableau du **second passage**, ms, chaque ligne101 observations :

| grille | mode/budget ms | pas avancés | médiane | p95 | maximum |
|---|---|---:|---:|---:|---:|
|16×8|sans limite|101|0,0842|0,1214|0,1422|
|16×8|coopératif1000|101|0,1150|0,1839|0,3927|
|16×8|0|0|0,0002|0,0004|0,0005|
|16×8|0,05|0|0,0505|0,0509|0,0517|
|16×8|0,5|101|0,1214|0,1782|0,2111|
|16×8|2|101|0,1272|0,1940|0,3837|
|32×16|sans limite|101|0,6822|0,9231|1,0442|
|32×16|coopératif1000|101|0,9299|1,3310|1,5439|
|32×16|0|0|0,0003|0,0004|0,0005|
|32×16|0,05|0|0,0504|0,0510|0,0617|
|32×16|0,5|0|0,5005|0,5014|0,7722|
|32×16|2|101|0,8122|1,3285|1,8767|
|64×32|sans limite|101|5,5125|7,5189|7,8800|
|64×32|coopératif1000|101|6,0760|7,0603|8,6546|
|64×32|0|0|0,0002|0,0005|0,0009|
|64×32|0,05|0|0,0506|0,0511|0,1083|
|64×32|0,5|0|0,5005|0,5018|0,5514|
|64×32|2|0|2,0008|2,0018|2,0732|

**Premier passage,64×32 limité2ms : médiane2,0010, p952,1491, maximum15,8501ms**, soit
13,8501ms de retard. Second passage :73,2µs de retard maximum sur ce même budget, et272,2µs
sur32×16 à0,5ms. La cause du pic n'est pas identifiée : ni suspension OS ni coût du cœur ne
peuvent être affirmés sans profil. La différence externe/dernier contrôle était au plus2,9µs
sur la ligne du premier pic ; le temps a donc été observé par le contrôle, pas caché dans un
rollback massif. Au second passage cette différence culmine à8,8µs tous cas confondus.

Le surcoût médian du chemin coopératif1000ms face au témoin est d'environ37%,36%,10% selon
la grille au second passage ; séries successives, bruit non isolé. Aucun gain de précision ni
promesse universelle de coût. **I-05 complet non reçu**, budget eau inchangé.

Un domaine qui n'achève jamais le pas sous son enveloppe n'avance pas : l'hôte doit l'admettre
avec une charge/résolution/fréquence viable, ou le dégrader/désactiver explicitement. Répéter le
même abandon ne constitue pas une progression. Le contrat de précision f32 reste le prochain
lot J2 autonome ; l'admission et les marges temporelles gardent leur déclencheur d'intégration.

Commandes depuis `code/` : `cargo test -p water-core --release --offline --test delta_runtime`,
`cargo run -p water-core --release --offline --example delta_budget_cost`,
`cargo run -p water-core --release --offline --example delta_filters` et suite workspace release.

Suite workspace release : **407 réussis, 5 ignorés** (trois nouveaux tests). Après le contrôle supplémentaire de colonne, sept tests runtime release et le filtre sont reçus ; les autres modifications sont documentaires. Navigation active sans erreur. Les avertissements du harnais/exemples sont préexistants.
