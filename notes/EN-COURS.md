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

Session : S645 — **terminée**. En autonomie, vers la v2 (« continue en autonomie jusqu'à la v2 », 2026-10-07). **A333** : le jet de rive
d'APIC 3D court d'un cinquième (S644).

**L'hypothèse.** Sur l'escalier, les faces des mailles du fond sont mises à zéro (`impose_body`). Le dessus des marches est donc glissant
pour la vitesse normale, mais l'interpolation de la grille vers les particules (`grid_to_particles`, trilinéaire) mêle à la vitesse
horizontale d'une particule, dans la moitié basse de la première maille d'eau, la vitesse **nulle** des faces `u`/`v` de la maille solide
dessous. Une lame d'une ou deux mailles est alors freinée comme contre une paroi non glissante.

**Le témoin (ADR-259 D1).** Le fond glissant (`set_seabed_slip(true)`) : les faces `u`/`v` de la maille solide du dessus d'une colonne,
quand elles séparent deux mailles solides, prennent la vitesse de la face de même position juste au-dessus. Une face de contremarche, qui
sépare une maille solide d'une maille d'eau, reste nulle (sa vitesse est normale). La face `w` du dessus reste nulle.

**Critères, écrits avant.** (1) Avec le fond glissant, la remontée de S644 lue par les particules : **A333 levée** si elle est à 10 % de
Saint-Venant 2D à 2,5 cm (0,240 m) et plus proche à 2,5 qu'à 5 cm ; sinon l'hypothèse est écartée et rapportée telle. (2) Le repos de
S639 (le témoin plat ≤ 1 cm/s, la pente mesurée) avec le fond glissant ; (3) le fond glissant par défaut seulement si (1) et (2)
tiennent ; les essais d'APIC 3D passent.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le fond glissant ; les mesures ; (1)–(3).
- [x] **P3** — preuve ; A333 ; rituel.

### Notes de reprise
- **P2 fini** — le fond glissant : sans effet (0,186 m aux deux mailles), écarté ; la durée : sans effet ; **l'air balistique** (la seconde
  hypothèse, nommée à la mesure) : 0,225 m à 2,5 cm (étiquettes), 6 % de Saint-Venant, 98 % de Synolakis, convergent — **A333 levée**.
  Actif partout, il casse S406 (le raccord des colonnes, −5,77 mm/s) : éteint par défaut, le rouleau l'active. 51 essais d'APIC 3D.
