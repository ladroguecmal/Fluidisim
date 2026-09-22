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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S325 — **terminée** (2026-09-23 01:31, P5 reportée). **Lot 5, le raccord dynamique** — une région en colonnes qui évolue à
côté d'une région en particules, alternance d'[ADR-188](../docs/adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »*, après S324 ; suite déclarée : le raccord dynamique.

**Ce que la session doit rendre possible.** La première eau qui vit **à la fois** en colonnes et en
particules, et passe de l'une à l'autre en cours de simulation — le geste qui fera consommer APIC par
δ, et le point 4.20 (changement de représentation en cours de simulation). Consommateur : le raccord
en 3D, puis la porte D et C20.

**Le montage, le plus petit qui soit dynamique.** Dans le banc APIC 2D, la moitié droite du bassin
est portée par des **colonnes** : une hauteur par colonne, transportée par les flux ouverts de la
grille — le modèle des colonnes de δ. Ses particules sont **réensemencées à chaque pas** depuis ces
hauteurs : elles ne servent qu'au transfert vers la grille et à la surface que voit la pression. À
la frontière : une particule libre qui entre dans la zone des colonnes est retirée et sa masse ajoutée
à la colonne ; la part sortante du flux de la grille est ensemencée en particules libres, le reste
reporté. La masse totale est la masse des particules libres plus celle des colonnes.

Critères, écrits avant le code :
1. **Masse** : particules libres + colonnes conservées à 10⁻¹⁰ près en relatif, par construction.
2. **Repos**, 5 s : vitesse maximale sous 1 cm/s (APIC seul : 4,4 mm/s) ; écart de surface à la
   frontière sous 0,2 maille.
3. **Ballottement**, 10 s, 5 et 2,5 cm : période à 1 % de celle d'APIC seul à la même maille ; écart
   de surface à la frontière sous 0,5 maille ; aucune divergence.
4. Rien dans le cœur ; APIC seul inchangé au bit.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le candidat hybride dans le banc : colonnes à droite, particules à gauche, échanges
  à la frontière.
- [x] **P3** — repos et ballottement, deux mailles : masse, période, écart à la frontière.
- [x] **P4** — preuve : section datée de [B10-APIC-S320](../docs/validation/B10-APIC-S320.md), avec
  « Reproduire » ; file, liste 4.20 si la mesure le permet.
- [ ] **P5** — S320 P5b : §5 bis au retour du calcul lancé à 20:11 — asynchrone. **Reportée** : encore
  en calcul à 01:31 ; le point daté de la file la porte.
- [x] **P6** — rituel.

### Notes de reprise

**P2 + P3 (01:30), fusion déclarée.** Candidat `hybride` et mode `raccord_dyn`. Défaut corrigé avant la
première mesure : une particule de colonne qui glisse à gauche aurait été gardée et comptée deux fois —
toutes les particules de colonne sont retirées après chaque pas. **Masse exacte** partout (≤ 1,3·10⁻¹⁵).
*Repos 5 cm* : écart à la frontière 0,002 maille ; vitesse max **1,4 cm/s** (APIC seul 4,4 mm/s) —
critère 2 manqué ; fuite à sens unique de 4,8·10⁻⁴ m² en 10 s. *Ballottement* : échanges dans les deux
sens (0,155 entré, 0,154 sorti à 5 cm) ; **écart à la frontière 1,81 et 2,85 mailles** (APIC seul 0,14 et
0,17) ; amortissement **16 % puis 5 % par période** (APIC 0,4 et 0,3 %) ; période +7,6 % zéros /
+5,2 % périodogramme à 5 cm, +2,4 / +0,14 % à 2,5 cm ; vitesses parasites 0,6 et 0,9 m/s. Critère 3
manqué. **Hypothèse du lissage grille → réseau → grille, contredite** : l'amortissement ne suit pas la
taille de la zone — 16 % (frontière à L/2), 0,9 % (3L/4), 10 % (7L/8). Cause non attribuée ; suspects :
l'insertion des particules sortantes, la quantification de l'ensemencement, la frontière au nœud du mode.

**P4.** Liste 4.20 **inchangée** : l'échange à masse exacte est reçu, la frontière non.
