# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S252 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo et Python disponibles.
Entrée : « continue ». Master propre 97868aa, une seule copie, archive B conservée.
Troisième session du raccordement B/W→δ : justifiée au journal S251 (chemin de J2).

Objectif. (1) A284 : attribuer les itérations du démarrage plat 32×16 par projection
(ordinaire, repli, affinage) ; diagnostic seul, aucune optimisation dans ce lot.
(2) A50/J2 : premier couplage B/W→δ à **surface géométriquement mobile**. `eta` porte
la hauteur perturbative η' ; la géométrie mouillée est celle de la surface totale
ζ = η' + ζ_fond ; la valeur fantôme vaut ρg(z_Γ − repos) − P_fond(Γ) ; la cinématique
ajoute le débit du fond entre le plan moyen et ζ (identité exacte si le fond est
linéaire, ζ_fond,t = W_fond(0)). Consommateur : nouveau pas perturbatif mobile.

Réception, écrite avant code. a) fond nul : identité au bit avec step_surface_mobile
sur une trajectoire ; b) repos exact ; c) onde stationnaire S237 (L=h=2 m) depuis le
repos, fond = ordre un analytique, δ né à zéro, contre HOS M=3 : profil ≤2 % de a à 128
colonnes et décroissant 32→128 (a = 5 et 10 cm), b₂ ≤20 % du maximum HOS à 128 et
décroissant ; chiffres S237 du solveur total en regard ; d) témoin sans résidus de
surface : b₂ hors tolérance ; e) expiration/refus atomiques, zéro allocation, suite et
empreinte. Un critère manqué est publié tel quel ; aucune tolérance déplacée.
Hors lot : frontières du total (fond B profond sur fond fini), prolongement de B
au-dessus du plan moyen en production, 3D, rendu, optimisation d'A284.

### Plan

**Amendement après P2 (2026-09-16), avant tout travail qui en dépend.** P2 a trouvé un
défaut du cœur : le β du gradient conjugué multigrille (A285). Il fausse le coût du repli,
A284 et la mesure S245 sur laquelle ADR-147 a fondé l'ordre « repli, pas ordinaire ».
Corriger d'abord ; la surface mobile couplée (anciens P3–P9) passe à **S253**, dérivation
conservée ci-dessous. Réception A285 : un test reproduit le défaut avant correction —
premier vrai résidu du chemin multigrille au-dessus du plancher atteint par le chemin
ordinaire sur le même système —, et passe après ; portes d'acceptation ADR-143/144
inchangées ; S245 re-mesurée aux cinq tailles, repli à 32 768 mailles, démarrage plat ;
suite, empreinte `delta_filters`, `delta_precision`. Aucune décision d'ordre dans ce lot :
un changement d'ADR-147 se décide sur la mesure publiée, au lot suivant.

- [x] **P1** — amorce, lectures et plan seuls.
- [x] **P2** — A284 : itérations et temps par projection, 32×16 plat, test diagnostic.
- [x] **P3** — A285 : test qui reproduit le défaut, correction de β, tests multigrille.
- [x] **P4** — re-mesures S245 (cinq tailles), 32 768 mailles, démarrage plat ; empreinte.
- [x] **P4b** — ADR-151 : affinage de divergence ADR-150 étendu au pas à couvercle fixe
  refusé au plancher (A275 sinon rouverte) ; test à 32 768 mailles, suite complète.
- [ ] **P5** — preuve, notes datées ADR-147/S245/S246/S251, angles morts, file.
- [ ] **P6** — rituel §6 : trajectoire, journal et jeton ; S253 = surface mobile couplée.

### Notes de reprise

Dérivation (pas encore dans un fichier). Fond linéaire : ζ_t = W(0) et, pour un
prolongement incompressible, R_s = ζ_t + U(ζ)ζ_x − W(ζ) = ∂x∫₀^ζ U dz exactement.
Donc η'_t = −∂x[∫_fond^ζ v dz + ∫₀^ζ U dz] : ni ζ_t ni échantillon de colonne requis ;
ζ_fond se lit dans `eta` de tout échantillon (indépendant de z). Fond du domaine sans
flux du fond (W(b)=0) supposé : sinon frontière du total, hors lot. B refuse z>0
(ADR-113) : le prolongement de Taylor d'ordre un (champ et dérivées cohérents) est
une convention de l'hôte, éprouvée ici par le seul fournisseur de test.
Transport : garder la somme S237 au bit, ajouter la bande à part (identité a).
Démarrage plat mobile : risque A283 (petit champ) ; ADR-150 ne couvre que le couvercle.
P2 (A284), test ignoré `flat_start_cost_attribution_s252`, release, 32×16 plat, 20 pas :
ordinaire 80–82 it / 0,84–1,36 ms, au plancher, D 1,55e-4→1,06e-5 (reçu dès le pas 17) ;
**repli multigrille 500–501 it / 37,7–42,9 ms** (94 % du pas), refusé pas 0–6 ;
affinage 86–92 it / 0,9–1,3 ms. Vrais résidus du repli : 0,616 à it 18, puis ×0,6 par
relance de ~20 it, 26 relances. Cause : β du GC multigrille = ‖r₊‖²/⟨r,z⟩ au lieu de
⟨r₊,z₊⟩/⟨r,z⟩ (`project`, branche multigrid_on, depuis S245 P5 `6dc0bfa`) : la direction
croît d'environ 4/dx² par itération et le pas s'arrête sur dq non fini. **Expérience non
committée** β correct : repli 9 it / 0,75 ms, vrai résidu 3,8e-6 ; pas 0 = 2,6 ms (41),
pas 7 = 1,7 ms. S245 (« la multigrille ne gagne pas de vitesse ») a mesuré ce GC fautif.
P3 : `multigrid_conjugate_gradient_keeps_its_recursion_s252` échoue avant (premier vrai
résidu 0,616 à it 18, 514 it, 25 relances ; plancher ordinaire 1,0735e-5 à 80 it) et passe
après (9 it, 4,03e-6, une relance). `multigrid_into_dir` reçoit ⟨r_n,z_n⟩ et forme β après
le cycle. Tests δ release : 50 réussis, 11 ignorés.
**Amendement P4 (avant P4b).** Re-mesures avant/après sur la même machine, secteur 99 %.
`delta_precision` : champs et CASE identiques hors temps ; empreinte `delta_filters`
0xfb12b2092df4ee6d inchangée. S245 « avec » (it / ms) avant → après : 128 : 94/2,12 → 6/0,157 ;
512 : 177/13,8 → 7/0,913 ; 2 048 : 158/50,1 → 7/3,34 ; 8 192 : 120/159 → 8/12,5 ;
32 768 : 106/595 D=7,27e-6 reçu → **8/88,9 D=1,585e-5 refusé**. « sans » (ordinaire puis repli)
32 768 : 880 ms reçu → 340 ms refusé. Plat 32×16 : médiane 43,6 → 2,13 ms, max 48,2 → 4,57.
L'acceptation A275 tenait aux relances du GC fautif (raffinement itératif accidentel), pas
à « moins d'itérations » (S245). Test ignoré `largest_grid_after_multigrid_fix_s252` : un
affinage ADR-150 (q multigrille, couvercle homogène) → 8 it, 46,1 ms, D=3,56e-8 reçu.
À 32 768 le GC multigrille (8 it ≈ 46 ms) bat l'ordinaire (425 it ≈ 250 ms) : ordre
d'ADR-147 à reprendre au lot suivant, sur mesure publiée — pas dans P4b.
P4b : `refine_divergence` partagée (coupling : multigrid=false, au bit) ; `run` l'appelle si
dégradé au plancher après repli ; `iterations` cumulées (plafond par projection, comme
ADR-150). Deux tests ajustés au compte réel (`a_tight_budget…` : 2 = ordinaire + repli ;
`global_allocator…` : ≤ 3·limite). `largest_grid_is_received_by_refinement_s252` (release) :
témoin multigrille seule refusé (8 it, D 1,585e-5), pas reçu (441 it, 385 ms, D 3,56e-8).
Suite release **451 / 17 ignorés / 0**. delta_precision identique hors temps, empreinte
inchangée. Table S245 finale (sans | avec, it/ms) : 128 30/0,087 | 6/0,142 ; 512 61/0,677 |
7/0,602 ; 2 048 114/5,04 | 7/2,44 ; 8 192 220/37,2 | 8/15,0 ; 32 768 441/392 reçu | 24/137 reçu.
Bancs couplés : 32×16 plat médiane 2,53 ms (max 3,51), 16×8 plat 0,279 (5 affinages),
imposés inchangés au bit (D max 9,6369e-6 / 5,7869e-6). Secteur 99 % avant et après.
