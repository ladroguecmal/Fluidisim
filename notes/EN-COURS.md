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

Session : S437 — **terminée**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-3a**, A320
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
- [x] **P3** — mesures : 25 cm, deux houles, quatre décalages ; le point fixe ; conclusion.
- [x] **P4** — preuve ; A320 ; registres ; suite.
- [x] **P5** — rituel.

### Notes de reprise
- **P2** — le banc `mer` : `MER_DECALAGE=<f>` (le repos `f·dx` au-dessus de la face, le germe posé dessus), `MER_TEMOIN=1` (le témoin
  avance en mode germe ; la trace dit `temoin_nul_au_bit`), `MER_AIR=<n>` (des mailles d'air de plus : à ¾ de maille sous 7,5 cm, la
  surface sort sinon des bornes du pas au premier pas).
- **P3 (en cours)** — 25 cm, germe de 1 mm, taux de la bande 35–59 s ; le témoin avance : **nul au bit à chaque seconde, partout**
  (A324 tient hors d'une face). Repos sur une face (0), ¼, au centre (½), ¾ : **6 cm** 0,077 · 0,066 · **0,018** · 0,080 (le domaine
  refuse à 64 s) ; **7,5 cm** 0,106 · 0,068 · **0,036** · décroît. `MER_AIR=1` au bit. **Le taux dépend de la place du repos dans la
  maille — une physique ne le ferait pas** ; au centre, le critère est tenu (6 cm : 1,0 fois Benjamin-Feir ; 7,5 cm : 1,3 fois).
  Amputations aux faces de surface (`set_cross_surface_trial`, `MER_SURFACE_ESSAI`) : tous les termes croisés · le seul cisaillement,
  sur la face 0,095 · 0,092, au centre 0,026 · 0,026 — **pas là**. La bande prend déjà la pente verticale de B (`band3`) — pas là non plus.
  Le régime « repos près d'une face » n'existe que sous une houle de moins d'une demi-maille ; une vraie mer balaie toutes les places :
  `MER_TP`, `MER_PROFONDEUR` — houle de 16 m, 30 cm (`ak` = 0,118, Benjamin-Feir 0,0137 s⁻¹, critère 0,021), 8 m d'eau, 25 cm,
  repos 0, ¼, ½.
- **P3** — la houle de 16 m (30 cm) abandonnée : le germe de 2 m n'y sème rien à `K`, et e-folder Benjamin-Feir y prend 73 s ;
  `MER_GERME_LAMBDA` (le germe à la longueur d'onde voulue), la trace de la bande en notation scientifique. **Houle de 8 m**
  (`MER_TP=4.527`, 4 m d'eau, germe de 8 m), 25 cm, 95 s : le germe sort en ≈ 30 s ; ensuite la bande ne croît qu'à **≈ 0,007 · 0,004**
  s⁻¹ sous 15 cm (repos sur une face · au centre ; Benjamin-Feir 0,019 — la surface de B franchit des centres dans les deux cas) et
  **≈ 0,009 · ≈ 0** sous 10 cm (Benjamin-Feir 0,0086 ; sur une face, aucun franchissement). **Au plus Benjamin-Feir, aux deux places.**
  **Conclusion.** Le critère, tel qu'écrit (la houle de 4 m du banc, toute place du repos), est **manqué** : 0,077 et 0,106 sur une
  face. Mais l'excès d'A320 dépend de la place du repos — un défaut de discrétisation près de la surface, pas une physique — et il
  **n'apparaît qu'à 16 mailles par longueur d'onde de la houle** (4 m à 25 cm) ; à 32 (8 m à 25 cm), rien au-delà de Benjamin-Feir.
  C7d-3a **non reçu** ; A320 ramenée à une houle mal résolue. Suite 758.
- **P4** — MER-S369 §9, APIC-CARTE §22.8 ; A320 annotée ; index, liste, feuille de route, questions ; suite 758.
- **P5** — journal ; jeton libre ; maillons 25 (justifiés : S406) ; suivant : S438, C7d-3a — localiser le défaut près de la surface.
