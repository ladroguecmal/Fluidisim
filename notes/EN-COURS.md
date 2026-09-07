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
Session          : S40
État             : terminée
Battement        : 2026-09-07
Objectif         : Trancher le seuil de sec — et d'abord établir de quoi il décide
```

### Plan

Action **S37-1**, angle mort **A163** *(sévérité 1)*, **reportée quatre fois**. Deux valeurs
coexistent : `H_SEC = 10⁻⁶ m` dans `delta.rs`, dont `ADR-031` §4 donne la provenance, et `10⁻¹⁰` en
dur dans `shallow.rs`, sans justification. Quatre ordres de grandeur.

> **Mais le corpus contient déjà deux mesures qui semblent se contredire, et c'est là qu'est la
> vraie question.**
>
> - **`ADR-031` §4** a balayé `H_SEC` sur **six décades** — `10⁻⁹` à `10⁻³` — et mesuré **0,25 point
>   d'effet sur seize** sur la position du front, volume identique dans tous les cas. Conclusion
>   écrite : *`H_SEC` n'est pas un paramètre physique, sa valeur est libre sur au moins six décades.*
> - **S37** a mesuré, sur le même cas, **6,16 m/s** d'écart de vitesse entre les deux véhicules —
>   **98 % de la vitesse du front de Ritter** — imputable à ce même seuil.

**Les deux sont vraies, et elles ne parlent pas de la même grandeur.** L'une mesure une **position de
front**, robuste ; l'autre une **vitesse dans le film**, où `hu/h` sur un dixième de nanomètre rend
n'importe quoi. La question n'est donc pas *quelle valeur choisir* — `ADR-031` a déjà répondu que la
valeur est libre — mais **de quoi ce seuil décide réellement**.

*Thèse déclarée, falsifiable en une mesure :*

> **Aucune grandeur publiée de C04 ne bouge de plus de 1 % quand le seuil varie de `10⁻³` à
> `10⁻¹⁰` — front, erreur `L¹`, `h(0)`, `u(0)`, volume. Seule `max|u|`, que rien ne publie, explose.**

**Si la thèse tient, la décision n'est pas de choisir un nombre** : c'est de dire que la vitesse dans
le film n'est pas une grandeur publiable, et d'aligner les deux valeurs pour une raison de
**comparabilité** et non de physique. **Si elle est fausse**, une grandeur du corpus dépend d'une
constante posée au jugé d'un côté, et il faut vraiment trancher.

### Trois objets portent le même nom, et c'est à clarifier avant tout

C'est **L148** — *un seuil coupe ce qu'il nomme, jamais ce qu'on croit qu'il nomme* — et la
confusion est déjà dans le corpus :

| | ce que c'est | où | valeur |
|---|---|---|---|
| **seuil du solveur** | sous lui, `u = 0` — garde-fou de division | `delta.rs`, `shallow.rs` | `10⁻⁶` / `10⁻¹⁰` |
| **seuil de mesure du front** | ce qui compte comme « mouillé » pour situer le front | `physics.rs`, `physics_shallow.rs` | `10⁻²·h₀` *(révisé en B-S25)* |
| **seuil de flux** | *n'existe pas* — la diffusion dépose sous les deux autres | — | **A165** |

**A163 porte sur le premier.** `ADR-041` a révisé le deuxième. Le troisième est un manque, pas une
valeur.

- [x] **P1** — plan, jeton.
- [x] **P2** — rendre le seuil de `shallow.rs` **paramétrable**. Il est en dur à deux endroits ;
      sans cela, rien ne se balaie de ce côté et l'oracle ne sert pas.
- [x] **P3** — **rejouer le balayage d'`ADR-031` §4** dans cet arbre, étendu à `10⁻¹⁰`, et sur les
      **deux** véhicules. Un chiffre publié se rejoue avant de servir (**L143**).
- [x] **P4** — mesurer ce que le seuil déplace sur **chaque grandeur publiée** de C04.
- [x] **P5** — et sur celles que rien ne publie : `max|u|`, la longueur du film, le compte de
      cellules litigieuses. C'est là que la sensibilité vit.
- [x] **P6** — **la décision**, en ADR. Elle porte sur ce que le seuil gouverne, pas sur un nombre.
- [x] **P7** — répercussions : `ADR-031` §4 reçoit ce que S40 ajoute, `A163` est tranché ou
      requalifié, `oracle.rs` cesse de porter deux constantes en dur.
- [x] **P8a** — rituel : journal, leçons L156-L157, actions S40-1 à S40-4.
- [x] **P8b** — rituel : index, décomptes, jeton libéré.

### Notes de reprise

**Ce que les sessions précédentes laissent et qui commande celle-ci.**

- **`ADR-031` §4 a déjà fait la moitié du travail**, et sa conclusion est à relire avant de
  mesurer : *une constante dont l'effet a été mesuré a une provenance, même quand l'effet est nul.*
  Le balayage s'arrêtait à `10⁻⁹` ; `shallow.rs` est à `10⁻¹⁰`, juste en dessous.
- **`delta.rs` a `avec_h_sec`** — le seuil y est déjà paramétrable. **`shallow.rs` ne l'a pas** :
  `1e-10` est écrit dans `vitesse()` et dans le flux HLL, plus six autres endroits (`> 1e-10`).
- **`oracle.rs` porte `SEC_DELTA` et `SEC_SHALLOW` en dur**, recopiées des solveurs en S37. Deux
  copies d'une constante qui doit devenir paramètre : à nettoyer en P7, sinon elles se périment en
  silence (**L141**).
- **Ce que cette session ne doit pas faire** : aligner les deux valeurs « au passage ». C'est
  précisément ce que `ADR-044` §7 interdit — *une session qui les alignerait au passage changerait
  une décision par une retouche de constante.* La décision se prend en ADR, après mesure.
- **État de départ** : `cargo test` = **84 tests** (38 cœur + 46 harnais, deux `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Mode `physics` : 33,7 s sur 60.
