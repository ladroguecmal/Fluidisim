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
- [ ] **P3** — fournisseur analytique de test partagé (exemple et essais) et ses contrôles.
- [ ] **P4** — cœur : hauteur totale, valeurs fantômes du fond, tampons comptés ; S237 au bit.
- [ ] **P5** — cœur : pas perturbatif mobile, transport de η' et bande, atomicité.
- [ ] **P6** — essais : identité au bit, refus, expiration, allocations, témoin sans résidus.
- [ ] **P7** — banc HOS couplé 32/64/128, 5 et 10 cm : réception ou refus publié.
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
