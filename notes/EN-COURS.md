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

Session : S690 — **terminée**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage, l'étape 4 : la vague qui plonge.

**Ce que la session fait.**

- **Le relais sur fond en escalier.** `RelaisRivage` lit le fond du bord 3D par l'escalier (`seabed_height`) quand le fond n'est pas
  lisse. La vague plongeante de S647 est validée sur l'escalier, avec l'air balistique.
- **La vague de S647** (pente 1:12, `S₀` = 0,231, plongeante) dans le relais à 2,5 cm. APIC va jusqu'à **10,775 m** (431 mailles), Saint-Venant au-delà.
  Elle est jugée contre le tout-3D de S647–S648 à la même maille (ADR-273 D1, la référence du côté d'APIC).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273)

- **témoin** : le tout-3D de S647–S648 à 2,5 cm : le premier retournement à 2,642 s et 9,988 m, l'air enfermé à 2,817 s et 10,375 m.
- **instrument** : les lecteurs de S647 et S648, sur la 3D du relais. Ce que rendrait chaque hypothèse :
  - si le raccord, à 0,40 m au-delà de la chute du jet, ne trouble pas le déferlement, le même retournement et le même air ;
  - si le raccord renvoie l'onde ou la freine, un retournement déplacé ou absent.
- **calcul** (ce script) :
  - à 5 cm, trois mailles de fond tombent à 9,9 m, **avant** la chute du jet (10,5 m) : l'étape ne se fait qu'à 2,5 cm ;
  - à 2,5 cm, à 10,775 m, sur 7,7 cm de fond (10,8 m n'a que 7,47 cm, refusé par le script), 0,40 m au-delà de l'air enfermé de S648
    (asserté).
- **ADR** : ADR-271 D1, ADR-273 D1.
- **pièges** :
  - l'air balistique avec la sortie à droite ;
  - le niveau à 0,55 m, exact sur le réseau des particules (0,55/0,0125) : une seule source, ADR-273 D2 ;
  - le coût (S647 : 23 min pour 3 s à 2,5 cm), depuis une copie du binaire.

**Critères, écrits avant.**

1. Le premier retournement du relais à moins de **0,02 s** et **0,15 m** du tout-3D (les bornes de S650).
2. L'air enfermé paraît après le retournement, en avant de lui.
3. La masse à 10⁻¹² près, la dette sous un quantum.
4. Rapportés : la remontée de Saint-Venant, sous la maille ; le temps de calcul contre le tout-3D.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le fond en escalier dans le relais ; l'essai ; (1)–(4).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2, en cours (notes)** — Le premier essai a tourné 8 h sans résultat (arrêté). Mesuré ensuite : 2,4 s d'horloge par pas sur un cœur, le
  pas de Saint-Venant figé à 2,4 ms. Accéléré : les 16 fils (`set_jobs`, au bit, S483) et le pas stable réel du relais (`pas_stable_us`) —
  2,5 s simulées en 5 min (≈ ×25). La progression affichée a montré **l'effondrement du pas après le déferlement** (163 µs à 3,0 s) ;
  deux causes suspectées au raccord (une éclaboussure qui soulève le niveau lu ; une colonne presque vide, `F/0,001`) ; le diagnostic
  tourne ; le remède candidat est prêt (scratchpad `s690_remede.py`, non appliqué). Le brouillon de la revue S691 (`adr274_brouillon.md`).
- **P2 fini** — (1) 2,624 s, 9,963 m (tout-3D 2,642 s, 9,988 m) ; (2) 2,777 s, 10,325 m ; (3) masse 1,2·10⁻¹⁶, dette sous un quantum ; (4) remontée 0,361 m, 13,6 min pour 4 s sur 16 fils. L'analyse par moments ; `outils/essai.py`.
