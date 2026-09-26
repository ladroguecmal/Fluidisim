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

Session : S398 — **en cours**. **C5b, première part** ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5 ;
conception §4.1 A1, §4.2) : **la zone des colonnes dans APIC 3D**. Demande de l'utilisateur (2026-09-27) : *« Continue »*
(objectif consigné en S397 : terminer le solveur). Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo,
Python ; ni carte graphique, ni Godot. Sert 4.16, 4.12, A316.

**Pourquoi là, et pas dans `Volume3`.** Le raccord demande **une seule projection** pour les deux représentations (§4.2 :
colonnes où la surface est un graphe, particules dans une bande, fluide fantôme sur `η` ou sur la surface reconstruite).
`Apic3` a déjà la projection à fluide fantôme, la reconstruction, les transferts, les parois et le corps ; `Volume3` porte des
modes nombreux (fond coupé, colonne graduée, couplage à B) qu'une bande heurterait. La référence la plus simple d'abord ; la
production (C7) aura son propre portage.

**Thèse.** Une **zone de colonnes** dans `Apic3`, activée par un masque de colonnes (`enable_columns`, réservée à la
configuration) : surface **`η` par colonne** — `φ = z − η`, sans reconstruction ni son biais (S323 : le biais dépend de
l'arrangement des particules, cause candidate de la migration de S397) ; vitesse **eulérienne**, gardée sur la grille d'un pas à
l'autre et **advectée** (semi-lagrangienne, la leçon de S397) ; `η` transporté par les débits mouillés, pris en amont, comme δ.
Sans masque, `Apic3` au bit. **Cette session : toutes colonnes, sans bande** — la machinerie éprouvée seule contre δ (`Volume3`,
pas mobile) sur la même cuve et le même instrument ; la bande et l'échange viendront ensuite (C5b, deuxième part).

**Critères, écrits avant.** (1) Sans masque, au bit : `apic3d_ballottement 10 0.05` imprime la ligne de S389 ; suite. (2) Toutes
colonnes, repos 2 s : vitesse ≤ 1 mm/s ; volume `Σ η·dx²` constant à 10⁻⁶ relatif. (3) Toutes colonnes, ballottements (1, 0)
(10 s) et (1, 1) (5 s) à 5 et 2,5 cm : période à **1 point** de celle de δ sur la même cuve ; amortissement par période ≥ 0 ;
publiés contre la période exacte.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `enable_columns` et le pas à colonnes dans `apic3d.rs` ; l'essai du repos ; critères 1 et 2.
- [>] **P3** — les ballottements, toutes colonnes, contre δ (même instrument) ; critère 3.
- [ ] **P4** — suite ; preuve `RACCORD-3D-S398` ; file, liste, feuille de route, index.
- [ ] **P5** — rituel.

### Notes de reprise
**P2 — `apic3d_columns.rs`** : `enable_columns` (masque, réserve comptée), `set_columns_surface`, `columns_volume` ; dans le pas,
quatre crochets inertes sans masque — vitesse du pas précédent gardée, advectée au pied de la caractéristique sur les faces de
la zone ; `φ = z − η` et étiquettes dans les colonnes ; transport de `η` par débits mouillés (hauteur moyenne des deux
colonnes, somme compensée, comme `transport_mobile3` de δ) ; pas stable et contrôle de finitude étendus. **Critère 1 tenu** :
`apic3d_ballottement 10 0.05` imprime la ligne de S389 au chiffre près. **Critère 2 tenu** : toutes colonnes, repos 2 s, vitesse
max **2,4·10⁻⁵ m/s**, volume à 7·10⁻¹⁵ ; une bosse de 5 cm qui se déploie 1 s garde son volume à 2·10⁻¹¹. Trois essais (refus
compris).
