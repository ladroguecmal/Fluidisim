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

```
Session          : S12
État             : en cours
Battement        : 2026-09-05
Objectif         : les quatre points « à spécifier » remontés par l'audit S11 §7.4
```

### Plan

Quatre sujets courts et indépendants, sans dépendance à un banc. Forme retenue : **un ADR unique**,
`ADR-023`, parce qu'ils ont une origine commune — l'audit — et qu'il faut dire lesquels partagent
aussi une *cause*, ce qui n'est pas la même chose.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — `ADR-023` §1–2 : la décision d'ensemble, puis le **terme d'impact** (*slamming*).
  *Thèse : la grandeur à publier n'est pas la pression de pic mais l'**impulsion de masse ajoutée**.
  La pression de pic est ce qu'on ne sait pas (elle diverge quand l'angle de carène tend vers 0) ;
  l'impulsion est un bilan de quantité de mouvement, qui ne peut pas être faux.*
- [x] **P3** — `ADR-023` §3 : le **nageur en surface**.
  *Thèse (L03) : il n'y a pas de modèle à écrire. Le mode contraint d'ADR-008 §3 existe déjà ; il
  lui manque une seconde condition d'entrée. Le critère actuel est de stabilité numérique, et le
  nageur le passe — c'est pour une autre raison qu'il en relève.*
- [x] **P4** — `ADR-023` §4 : les **sites turbulents permanents**.
  *Thèse : ce ne sont pas de nouveaux objets. Un site turbulent est une polyligne de déferlement
  dégénérée en un point, et il se dérive du critère `H/h = 0,78` déjà posé en SPEC-001 §3.*
- [x] **P5** — `ADR-023` §5 : la **coalescence des poches d'air T2**.
  *Thèse : la règle tient en une addition, parce qu'ADR-015 §3 a eu la bonne idée de stocker
  `n_moles` plutôt que seulement pression et volume.*
- [x] **P6** — `ADR-023` §6–7 : ce que chaque section ferme, ce qui reste ouvert ; puis les notes
  de clôture dans ADR-008 §5.3, §5.4, ADR-013 §7.4 et ADR-015 §7.3.
- [ ] **P7** — index, angles morts, cas canoniques, registre S11 (statut des quatre points).
- [ ] **P8** — rituel de fin (`REPRISE.md` §6) : journal S12, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- **Cause commune cherchée avant d'écrire, trouvée pour deux des quatre.** Le terme d'impact et le
  nageur sont les **deux frontières du domaine de validité d'ADR-008** : l'une dans le temps —
  l'impact est plus bref que le tick — l'autre dans la nature du corps — un nageur n'est pas passif.
  Les deux autres n'ont pas de cause commune avec eux, et il vaut mieux le dire que de forcer une
  unification qui n'existe pas.
- **Vérification faite avant de rédiger** : un nageur passe le critère de stabilité d'ADR-008 §3.
  `A_flottaison ≈ 0,25 m²`, `k = ρgA ≈ 2 450 N/m`, `m + m_a ≈ 145 kg` → `ω ≈ 4,1 rad/s`,
  `ω·dt ≈ 0,14` à 30 Hz, soit « intégration normale ». Le mode contraint ne se déclenche donc pas
  pour lui aujourd'hui, alors que c'est le mode qu'il lui faut. Le critère est bon, il est
  simplement le seul.

#### P2 — terme d'impact

Chiffres posés : `t_impact = 2b·tanβ/(πv)` donne **73 ms** pour une étrave de vedette (2,2 ticks) et
**17 ms** pour un corps humain (0,5 tick). L'impact dure de l'ordre du tick ou moins — un terme
échantillonné à 30 Hz le rate ou le double selon la phase.

`C_p = 1 + (π/2tanβ)²` : **c'est l'angle qui domine**, pas la vitesse. 30° → 10° multiplie la
pression par dix ; doubler la vitesse ne la multiplie que par quatre. Et la formule **diverge quand
β → 0**, donc on ne connaît pas la pression de pic.

D'où la décision : publier l'**impulsion de masse ajoutée** `J = Δ(½πρc²)·v_rel`, qui est un bilan
de quantité de mouvement, que l'intégrateur du solide sait appliquer exactement, et qui réutilise le
tenseur de masse ajoutée déjà imposé par ADR-008 §2. Contrôle croisé fait : 1,1 MN moyens sur 73 ms
concordent avec les 105 kPa de pic sur la surface mouillée.

#### P3 — le nageur : la question se dissout (L03)

Pas de modèle à écrire. Le mode contraint d'ADR-008 §3 existe ; il lui manquait une seconde
condition d'entrée — *un corps contrôlé, en surface, y bascule quel que soit son ω·dt*. Le critère
existant mesure la stabilité numérique, et la stabilité n'est pas le problème du nageur : c'est le
contrôle et la caméra.

Deux seuils **dérivés**, que le design n'aura pas à choisir :
- `πH/T = 0,7 m/s` → par mer de **1 à 2 m un nageur ne va plus où il veut** ;
- un nageur ne décolle de la surface que sous un **rouleau plongeant** — l'accélération de crête de
  SPEC-002 §1 (g en plongeant) est exactement le critère, écrit pour autre chose.

Contrôle fait : les cinq effets qui comptent (emportement, eau blanche, déferlante, hypothermie,
seuils de progression) passent tous par d'autres chemins déjà spécifiés. Le mode contraint ne coûte
rien au gameplay.

#### P4 — la spécification corrige la proposition d'origine

ADR-013 §7.4 disait « émetteurs W stationnaires ». Le mot *émetteur* ne survit pas au chiffrage :
200 sites × 1 év/s × 45 o = **9 000 o/s par joueur, dix fois une bataille navale, en permanence,
pour du décor**. Et un phénomène stationnaire déterministe n'a aucune raison d'être répliqué.

Décision : **terme stationnaire dérivé**, re-calculé à la demande — exactement ce qu'ADR-014 §2.3
avait déjà retenu pour l'écume permanente, dans un autre document et sous un autre nom. ADR-013 §7.4
proposait donc un second mécanisme sans le savoir.

Critère de dérivation : `h < 1,28·H_local` (McCowan). Bénéfice non demandé : avec 4 m de marnage,
un rocher à 3 m **brise à basse mer et pas à haute mer**, par mer de 2 m. Un récif qui gronde deux
fois par jour à heure prévisible, sorti d'une inégalité.

#### P5 — coalescence : `n_fusion = n_a + n_b`

La règle tient en une addition **parce qu'ADR-015 §3 avait stocké `n_moles`** et pas seulement
pression et volume. Le champ n'avait pas été introduit pour cela ; il rend l'opération triviale huit
sessions plus tard. L06 en petit.

Le volume fusionné sort d'une dichotomie sur le `shape_lut`, **monotone par construction** — six
itérations pour 64 entrées. La même monotonie sert de test d'étanchéité en SPEC-005 §9 : second
usage gratuit d'une seule décision.

Deux ajouts non demandés par le point ouvert : la **scission** (moles au prorata des volumes — toute
autre répartition fait sauter la poussée d'une coque retournée quand une cloison émerge) et
l'**hystérésis** sur la hauteur d'ouverture, ε = 5 cm, même parade qu'ADR-010 §5 pour les flaques.

#### P6 — bilan : quatre mécanismes, zéro interface nouvelle

Ni le terme d'impact ni le mode contraint n'ajoutent d'interface : la flottabilité est calculée
**du côté hôte** à partir de `EvalWaterBatch`, qui fournit déjà `eta`, `normal` et `u_total`. Le site
turbulent réutilise le canal de SPEC-006 §6 inchangé ; la coalescence est interne à V.

Confirmation au passage de la séparation d'ADR-008 §1 : le terme d'impact étant **autoritaire**, il
n'emprunte pas `accumulate_force`, qui ne mène qu'à la pose de rendu — il n'aurait pas pu, ce qui est
exactement l'effet recherché quand I-04 a été rendu mécanique (L19).

*Correctif P6* : la clôture d'ADR-013 §7.4 n'avait pas trouvé sa cible — le retour à la ligne de la
note S11 différait de ce que le script supposait. Le script rapporte ses échecs, l'écart a été vu
immédiatement et corrigé par un commit séparé plutôt qu'en réécrivant l'historique.
