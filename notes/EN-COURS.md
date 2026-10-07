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

Session : S650 — **terminée**. En autonomie vers la v2 ; le rouleau 3D. **Étape 4 — le relais 2D → 3D** : la 3D seulement là où elle sert.
Le lot des registres (S648–S650) à la fin.

**Ce que la session fait.** Le montage de S647 (onde solitaire `H/d` = 0,3, pente 1:12) coupé en deux à `x_r` = 5.0 m (0,7 m avant le
pied) :

- **Saint-Venant 2D** (S613, ordre deux de S620) porte l'onde sur toute la plage, depuis le même état initial ;
- **APIC 3D** ne couvre que `[x_r ; 12,8]` m. Une **zone de colonnes** (S398) borde le large, du bord à 0,6 m ; les **particules**
  occupent le reste, sur la pente, avec l'air balistique (S645). Son bord gauche est **ouvert** (S446) : il reçoit à chaque pas la
  vitesse de Saint-Venant à `x_r` (`q/h`, uniforme sur la verticale), et la zone de colonnes compte le débit qui entre. Le bord droit
  reste un mur ;
- l'état initial de la 3D est celui de l'onde : `η` des colonnes, la vitesse de la grille ; les particules au repos, comme en S647
  (à `x_r`, `η` = 0.0262 m).

**Critères, écrits avant** (le comparant : le tout-3D de S647–S648 à 5 cm, retournement à 2,571 s et 9,675 m, air enfermé à 2,872 s et
10,525 m).

1. **La masse** : le volume de la 3D (colonnes et particules) varie du volume entré par le bord, au millionième du volume total.
2. **À 5 cm, le relais se retourne avant le rivage**, à 0,15 m et 0,1 s du tout-3D.
3. **De l'air enfermé apparaît après le retournement, en avant de lui** (S648).
4. **Rapporté** : le temps de calcul contre le tout-3D, mesuré au même instant, sur la même machine.

L'écart attendu : Saint-Venant, sans dispersion, raidit l'onde sur 1,6 m ; un retournement un peu plus tôt est la signature du relais, non
une faute. Les deux lecteurs (S647, S648) sont réemployés tels quels.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le relais ; (1)–(4).
- [x] **P3** — preuve ; liste 4.14 ; le lot ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(3) tenus : volume − entré −3,7·10⁻¹⁷ ; retournement 2,575 s, 9,825 m (tout-3D 2,571 s, 9,675 m) ; air enfermé 2,667 s,
  10,075 m ; (4) 163 s contre 256 s (64 %), chronométrés dans la même exécution.
