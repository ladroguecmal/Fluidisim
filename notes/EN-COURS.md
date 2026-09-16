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

- [x] **P1** — amorce, lectures et plan seuls.
- [ ] **P2** — A284 : itérations et temps par projection, 32×16 plat, test diagnostic.
- [ ] **P3** — ADR-151 et protocole de réception, avant code.
- [ ] **P4** — fournisseur analytique de test (onde stationnaire linéaire, prolongement).
- [ ] **P5** — cœur : géométrie totale, valeurs fantômes du fond, tampons comptés.
- [ ] **P6** — cœur : pas perturbatif mobile, transport de η' et bande du fond, atomicité.
- [ ] **P7** — tests : identité au bit, repos, expiration, allocations, témoin.
- [ ] **P8** — banc HOS couplé 32/64/128, 5 et 10 cm : réception ou refus publié.
- [ ] **P9** — suite, empreinte, coût et preuve.
- [ ] **P10** — rituel §6 : file, trajectoire, journal et jeton.

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
