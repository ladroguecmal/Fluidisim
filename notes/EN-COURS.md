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

Session : S464 — **terminée**. En autonomie (décision du 2026-10-03) : le lot des registres (ADR-213 D3, dû en S460), puis **le direct**.

**Ce que la session fait.** (1) Le lot des registres pour S457–S463. (2) **Le direct** : la scène du saut dans Godot **sans
enregistrement**. **Tranché (ADR-215 D2)** — un lien local plutôt qu'une bibliothèque dans le processus de Godot (godot-rust demande une
dépendance que le dépôt ne porte pas, hors ligne) : `water-viewer --v1-direct` calcule la scène au temps réel sur la carte et pousse
chaque image (le champ `φ` sur 8 bits, la carte des caustiques, l'instant, le corps) sur `127.0.0.1:47011` ; `saut.tscn -- --direct`
s'y connecte et affiche la dernière image reçue. Godot ne calcule toujours rien (I-01).

**Critères, écrits avant.** (1) la scène tourne dans Godot, l'afficheur sans fenêtre, sans fichier entre eux ; (2) le temps simulé suit
le temps réel (0,9 au moins) et Godot reçoit 25 images/s au moins ; (3) une capture du direct montrée.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le lot des registres.
- [x] **P3** — le direct (`--v1-direct`, `--direct`) ; mesures ; capture.
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — feuille de route, liste, file active pour S457–S463 (la v1 reçue, C10-2 plafonné, C11) ; `Registres` : dernier lot S464, le prochain au plus tard en S467.
- **P3** — `water-viewer --v1-direct` (la scène au temps réel sur la carte, poussée sur `127.0.0.1:47011` : `FST1` et l'en-tête,
  puis `IMG1`, l'instant, le corps, `φ` sur 8 bits, les caustiques) ; `saut.tscn -- --direct` (la connexion, la dernière image complète
  appliquée) ; `encoder_image` commun à l'export et au direct ; `SurfaceCarte::demander_couches` / `couches_pretes` (la relecture sans
  attente, deux tampons). **Mesuré**, Godot affiché : d'abord **0,55** du temps réel et 13 à 19 images/s — la relecture du champ entier
  (15 ms), l'encodage (12 ms) et l'envoi (2 ms) dans la boucle ; encodage et envoi sur un fil à part : 0,78, 23,5/s ; la relecture sans
  attente : **0,3 ms** ; puis un défaut : **`φ` reçu nul** (le fondu lit ses dimensions dans l'uniforme, jamais écrit sans `set_view`)
  — la vue posée au départ : **0,939 du temps réel, 28,2 images/s** — (1), (2) tenus ; (3) captures `godot/captures/saut_direct_{0,1,2}.png`
  (le joueur qui remonte, l'eau qui ruisselle) envoyées.
- **P4** — C10-SCENES-S454 §13 ; journal ; jeton libre ; maillons 0 — capacité reçue (le direct : la simulation jouée dans Godot en temps réel ; le chemin, la pluie et R39 ; la preuve, §13) ; suivant : S465, la pluie sur la scène du saut.
