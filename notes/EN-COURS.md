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

Session : S437 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-3a**, A320
([preuve](../docs/validation/MER-S369.md) §7–8).

**Le critère de C7d-3a, réécrit avant toute mesure.** L'ancien (« taux < 0,01 s⁻¹ ») était hors d'atteinte de toute physique sous
7,5 cm (S435). **Nouveau** : C7d-3a est reçu si, **à la maille de la production (25 cm)**, germe de 1 mm, masque 7, 95 s, le taux de
l'amplitude de δ dans la bande de Benjamin-Feir (35 à 59 s) est **au plus 1,5 fois `ω(ak)²/2`** sous 6 et 7,5 cm de houle (0,026 et
0,041 s⁻¹), **quelle que soit la place du repos dans la maille** ; δ nul reste nul au bit ; le paquet de l'ordre C sous 5 cm reste sous
1,5 fois son amplitude (S434). Aujourd'hui : 0,077 et 0,106 — manqué d'un facteur 3.

**Ce que la session trouve en entrant.** À 12,5 cm, A320 croît à 0,054 s⁻¹ sous 6 cm (la surface de B ne franchit aucun centre de
maille) mais à **0,028 sous 6,5 cm** (elle en franchit) — deux fois moins vite pour une houle plus forte. Un comportement aussi
discontinu à la demi-maille dit qu'un des deux régimes est faux. Hypothèse : celui où la surface de B reste dans la maille de
surface — le régime de toutes les mesures d'A320 à 25 cm, où le repos tombe sur une face.

**Ce que la session fait.** (1) `MER_DECALAGE=<f>` : le repos déplacé de `f·dx` dans la maille (0 : sur une face, le banc d'avant au
bit ; 0,5 : au centre — la surface de B franchit alors un centre à toute amplitude) ; `MER_TEMOIN=1` : le témoin avance aussi en mode
germe et la trace dit s'il est nul au bit. (2) À 25 cm, sous 6 et 7,5 cm : décalages 0, ¼, ½, ¾. Le point fixe y est mis à l'épreuve
(A324 le garantit désormais hors d'une face). (3) Conclure : si le taux tombe sous le critère au centre et pas sur la face, le régime
« sans franchissement » porte l'excès — le localiser ensuite ; sinon, écrire ce qui est écarté.

### Plan

- [x] **P1** — jeton, plan seul, le critère réécrit.
- [x] **P2** — `MER_DECALAGE`, `MER_TEMOIN` au banc.
- [ ] **P3** — mesures : 25 cm, deux houles, quatre décalages ; le point fixe ; conclusion.
- [ ] **P4** — preuve ; A320 ; registres ; suite.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — le banc `mer` : `MER_DECALAGE=<f>` (le repos `f·dx` au-dessus de la face, le germe posé dessus), `MER_TEMOIN=1` (le témoin
  avance en mode germe ; la trace dit `temoin_nul_au_bit`), `MER_AIR=<n>` (des mailles d'air de plus : à ¾ de maille sous 7,5 cm, la
  surface sort sinon des bornes du pas au premier pas).

