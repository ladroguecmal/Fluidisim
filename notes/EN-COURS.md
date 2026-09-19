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

Session : S288 — en cours
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : continuer ; premier candidat GPU du calcul de pression.
Objectif : construire et éprouver opérateur mobile et lissage résidents GPU contre le cœur.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [x] **P2** — ADR du candidat expérimental ; export préalloué des coefficients du véritable
  opérateur mobile, réception contre son application native.
- [x] **P3** — noyaux GPU opérateur et Jacobi, ressources réservées et alternance de tampons,
  banc sur géométries planes/coupées/ondulées et tailles différentes ; découper si nécessaire.
- [ ] **P4** — mesures précision/coût complet/allocations, suites pertinentes et limites ;
  ni cycle complet ni simulation intégrée revendiqués avant leur construction.
- [ ] **P5** — rituel §6 : preuve, journal, registres/index/feuille, jeton libre.

### Notes de reprise

Troisième lot coût comparé à la 3D et aux solides : ces capacités manquent, mais ajouter une
seconde direction au solveur déjà >10 fois le budget accroît ce blocage. On construit ici le
premier étage GPU consommable par le futur solveur, sans prolonger les micro-optimisations CPU.
Cœur sans dépendance, pile wgpu déjà autorisée, δ cosmétique, aucune nouvelle source réseau.
Critères : export au bit contre opérateur CPU natif ; GPU contre CPU, erreur relative max
normalisée <=1e-5 sur opérateur et 32 lissages (précision de port, pas seuil physique).
Repos nul exact, bords/air/solides compris, pas de lecture CPU entre lissages. Coût de bout
en bout avec transferts et attente, GPU seul si horodatage disponible ; allocations publiées.
Seuil 1e-5 vise à distinguer une erreur de stencil d'arrondis f32 ; ne reçoit ni hauteur 3 mm
ni acceptation ADR-144 du solveur complet. Intégration future conditionnée à ces deux portes.

P2 : export natif reçu au bit sur fonds plans/coupés et surfaces ondulées, grilles
16x12 et31x19 ; refus atomiques forme/géométrie. ADR-172 acte seulement le candidat.

P3 : export consommé par kernels GPU opérateur/Jacobi ; 30 cas GPU passent, repos exact,
pas de readback entre 32 lissages. RTX5070/DX12. Première erreur max 1,69e-7 ; à128x52
32 lissages ~0,8 ms aller-retour contre2,5–2,9 ms CPU empaqueté. Non encore coût avec
export/empaquetage à chaque appel : P4 élargit cette mesure. 65–105 allocations pile+banc,
aucune prétention I-06 image. Noms WGSL réservés operator/smooth corrigés avant réception.
