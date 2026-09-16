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

Session : S253 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo et Python disponibles.
Entrée : « continue ». Master propre 7e3e1d8, une seule copie, archive B conservée.
Quatrième session du raccordement B/W→δ : justifiée au journal S252 (chemin de J2 et B4).

Objectif (A50/J2) : premier couplage B/W→δ à **surface géométriquement mobile**, consommé par
un nouveau pas `step_perturbation_mobile`. `eta` = repos + η' (née à zéro, I-12) ; géométrie
mouillée totale ζ = η' + ζ_fond, ζ_fond lu dans `eta` des échantillons ; valeur fantôme
p'(Γ) = ρg(z_Γ − repos) − P_fond(Γ), P_fond de la face voisine corrigé au premier ordre ;
cinématique η'^{n+1} = η' − (dt/dx)Δ[Q_v(ζ) + bande], bande = flux de U entre le plan moyen
et ζ. Exact pour un fond **linéaire** (ζ_t = W(0)), prolongé de façon incompressible au-dessus
du plan moyen, sans flux au fond du domaine. Dérivation : journal S252.

Réception, écrite avant code (détail au protocole P2). a) fond nul : identité au bit avec
step_surface_mobile ; b) refus et expiration atomiques, zéro allocation ; c) onde stationnaire
S237 (L = h = 2 m) depuis le repos, fond = ordre un de profondeur finie prolongé
analytiquement, δ né à zéro, contre HOS M=3 : critères S237 — profil ≤2 % de a à 128 colonnes
et décroissant 32→128, b₂ ≤20 % du maximum HOS à 128 et décroissant (a = 5 et 10 cm) ; S237
total en regard ; d) témoin sans résidus de surface (commutateur de test) : b₂ hors 20 %
contre l'ordre deux analytique. Critère manqué publié tel quel ; aucune tolérance déplacée.
Hors lot : frontières du total (W(fond) ≠ 0), prolongement de B en production (ADR-113 refuse
z > 0), multigrille/affinage du mode mobile, 3D, rendu, ordre d'ADR-147.

### Plan

- [x] **P1** — amorce, lectures et plan seuls.
- [x] **P2** — ADR-152 et protocole de réception, avant code.
- [x] **P3** — fournisseur analytique de test partagé (exemple et essais) et ses contrôles.
- [x] **P4** — cœur : hauteur totale, valeurs fantômes du fond, tampons comptés ; S237 au bit.
- [x] **P5** — cœur : pas perturbatif mobile, transport de η' et bande, atomicité.
- [x] **P6** — essais : identité au bit, refus, expiration, allocations, témoin sans résidus.
- [x] **P7** — banc HOS couplé 32/64/128, 5 et 10 cm : réception ou refus publié.
- [x] **P7b** — ADR-153 : affinage de divergence au plancher étendu au pas couplé mobile
  (valeurs fantômes homogènes) ; essai au premier pas à 128 colonnes, témoin refusé.
- [ ] **P7c** — réception complète 64/128 aux deux amplitudes, coût séparé sans concurrence.
- [ ] **P8** — suite, empreinte, coût et preuve.
- [ ] **P9** — rituel §6 : file, trajectoire, journal et jeton.

### Notes de reprise

Chiffres S237 total mobile (profil / b₂) : 10 cm 1,714/1,71 % (32), 0,592/0,98 (64), 0,230/0,43
(128) ; 5 cm 0,850/2,44 (32), 0,550/1,41 (64), 128 reçu S238 à 0,252/0,71. HOS : `examples/
support/nl_surface.rs` ; ordre deux fermé `second_order_b2` dans `examples/delta_mobile.rs`.
Prolongement de Taylor d'ordre un **non incompressible** (div = z·U_xz) : prolongement analytique
du mode de profondeur finie pour l'oracle. Lectures géométriques de `eta` : toutes dans
`delta_mobile.rs` (wet, ghost_up, ghost_side, surface_in_bounds, transport) ; `lid` et le
transport linéaire lisent `eta` hors mode mobile. Identité a) : somme S237 au bit, bande ajoutée
à part ; ajout des valeurs du fond seulement en mode couplé.
P4 : `height(i)` (η ou ζ totale), fantômes + `ghost_bg_up`/`ghost_bg_side` en mode couplé,
`prepare_surface_background` ; +(nu + 2·nx) f32 comptés, essai de comptabilité mis à jour.
Essai fantômes contre P analytique à l'interface : 32 colonnes vertes, 9 latéraux, borne de
Taylor 1,35 Pa. Tests δ 53/11 ignorés, exécution 11. `delta_mobile essai` = S237 (0,850 % / 2,44 %).

P5 : step_perturbation_mobile et transport_coupled ; surface_in_bounds et extrapolate_mobile
passent en pub(super). Essai critère 1 (fond nul) : 50 pas identiques au bit à S237, rapport compris.

P6 : essais refus (contexte, forme, non planaire, eta de colonne, garde) et expiration (5 coupures),
allocation nulle (pas et expiration). Témoin critère 5 : premier témoin fautif (éteignait −P_fond
latéral en entier, ordre un) — divergence 0,70 m/s au pas 100, refus Domain au pas 1145 ; corrigé
(pression du fond linéarisée ρg·ζ_fond(x_Γ), bande éteinte), note datée au protocole. Résultat :
couplé b₂ à 2,17 % de l'ordre deux, témoin 99,56 % (aucune harmonique : S volumique est un gradient).
Tests δ 57/11 ignorés, exécution 12.

P7 : banc `delta_mobile couple` (mode Coupled, fond analytique partagé), `couple_cas <mode> <a> <nx>`,
`DELTA_MOBILE_PAS` pour le coût. Premier lancement série arrêté (35 min) : cas couplés relancés en
trois processus (précision indépendante de la concurrence ; coût à mesurer à part). Contre HOS
(profil / b₂) : 5 cm 32 → 1,214 % / 1,62 % ; 64 → 0,276 / 0,54 ; 10 cm 32 → 0,908 / 1,51.
**Refus `Convergence` au pas 1** : 10 cm/64, 5 cm/128, 10 cm/128. Test ignoré
`coupled_mobile_first_step_refusal_diagnosis_s253` : 128/5 cm dégradé au plancher, D_franche
1,53e-5, vitesse corrigée max 1,1e-4 m/s (A283 en mode mobile) ; 64/10 cm reçu en contrôle
illimité (D_franche 8,1e-6) mais refusé sous budget (réductions découpées) : marge. Total S237
refait identique à 32/64 (0,850/2,44 ; 0,552/1,41).

**Amendement avant P7b.** Le protocole prévoyait de s'arrêter au diagnostic. Le remède est
une technique déjà reçue deux fois (ADR-150 couvercle couplé, ADR-151 couvercle fixe) : l'étendre
au pas couplé mobile est une décision bornée, déclarée ici avant tout code. Le pas S237 total
n'est pas touché (aucun refus observé). Critères de P7b : sans affinage, le premier pas à
128/5 cm reste refusé ; avec, reçu, `refinements = 1`, D_franche ≤ 1e-5, mode homogène éteint à
toute sortie. P7c : critères 3 et 4 du protocole inchangés.

P7b : homogeneous_ghost (valeurs fantômes nulles) posé par refine_divergence avec homogeneous_lid ;
step_perturbation_mobile affine si dégradé au plancher. Essai 128/5 cm (release) : témoin refusé
(690 it, D_franche 1,53e-5), pas reçu (1 145 it, D 6,67e-8), expiration tardive intacte. Tests δ
58/12 ignorés, exécution 12, S237 essai inchangé. ADR-153 écrit.
