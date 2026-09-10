# S143 — Ce qui garde le contrat de pente, et ce qui n'en a que l'air

2026-09-10. Traite **A210**, ouverte par S142.

## 1. Le défaut, énoncé précisément

Le crate compare cinq fois quelque chose à `max_slope` : deux constructions —
`RadialImpact::new`, `ImpactField::new` — et trois budgets — `composition`, `mixed_water`,
`bound_pressure`. Depuis S141-S142, **toutes** comparent une pente réelle, chaque champ divisant
sa borne L1 par un rapport qui lui est propre : 1,795071 pour la quadrature de Hankel, 1,701591
pour les 40 modes cartésiens. Ces deux nombres **ne se déduisent pas l'un de l'autre**.

Le contrat — *ce qui est comparé à `max_slope` est une pente réelle* — ne vit nulle part ailleurs
que dans deux commentaires et deux essais homonymes. Un troisième champ écrirait
`if slope > medium.max_slope` sans que rien ne l'arrête, et c'est exactement ce que les deux
premiers ont fait pendant soixante sessions.

## 2. La pesée, faite avant d'écrire

La question qui décide n'est pas « qu'est-ce qui est le plus propre » mais **« qu'est-ce qui aurait
arrêté la faute de S141 »** — une migration qui laisse un site en arrière — et **« qu'est-ce qui
échouerait le jour où quelqu'un l'oublie »**.

### Voie 1 — le type porteur, écartée, et le motif a inversé ma préférence

`Medium::max_slope: RealSlope` au lieu d'un `f32` nu, la borne L1 dans un `L1Bound` qui ne se
convertit qu'en donnant son rapport. Garde à la **compilation**, sans rien à inscrire : c'est ce
qui la rendait attirante.

Elle ne tient pas, et pour une raison précise : **l'hôte doit pouvoir construire un `RealSlope`**,
puisque c'est lui qui fournit le milieu. Le constructeur est donc public, et un troisième champ
écrira `RealSlope::new(slope)` — trois mots de plus — pour faire compiler sa comparaison fausse.
La garde devient une bosse, pas un mur. Elle réduit la probabilité de la faute ; elle ne
l'empêche pas.

Coût, mesuré et non estimé : **36 constructions de `Medium`**, 5 comparaisons, 4 contrôles de
validité, 3 lectures arithmétiques dont un hachage, 4 mutations d'essai — une cinquantaine de
sites, et une API publique changée. **Cinquante sites de bruit pour une bosse.**

### Voie 2 — l'essai générique sur les champs, retenue

Le même essai bout à bout que S141 et S142 ont écrit chacun de leur côté, appliqué à chaque champ :
dichotomie sur l'énergie jusqu'au dernier champ admis avec `max_slope = BREAKING_SLOPE`, puis
mesure de la pente réelle. Il aurait **échoué immédiatement en S141** sur `ImpactField`, ce qu'aucun
essai d'alors ne faisait.

Sa faiblesse est nette : un troisième champ n'y est pas soumis tant que personne ne l'y inscrit.

### Voie 3 — le recensement des sites, retenue, et c'est elle qui couvre la faiblesse de la 2

Un essai qui **lit les sources du crate** et vérifie que toute comparaison à `max_slope` figure
dans une liste connue. Il ne juge pas le calcul : il constate l'apparition d'un site. Un troisième
champ le fait échouer **le jour où il est écrit**, sans que personne ait rien à inscrire d'autre
que la ligne qui le déclare — et cette ligne oblige son auteur à dire quel rapport il applique.

C'est la seule des trois qui attrape ce qu'A210 décrit vraiment : non pas un calcul faux, mais
**une implémentation de plus qui ignore le contrat**.

### Voie 4 — l'entrée d'invariant, retenue en complément et jamais seule

S142 l'avait notée : `I-14` a tenu soixante sessions **parce qu'un essai le vérifiait**, pas parce
qu'il était écrit. Un invariant sans garde exécutable est une intention. Un invariant **avec** la
garde dit *pourquoi* la garde existe, ce qu'aucun code n'exprime.

## 3. Ce qui est construit

Voies 2, 3 et 4 ensemble — l'essai générique pour ce que les champs calculent, le recensement pour
ce que le crate contient, l'invariant pour dire ce que les deux protègent.

## 4. Les deux gardes, et la vérification qu'elles gardent

### Garde 1 — `every_field_places_its_limit_at_stokes_steepness_s143`

Le même essai bout à bout pour chaque champ, **écrit une fois** : S141 et S142 l'avaient écrit
deux fois, dans deux modules, et deux textes qui se ressemblent finissent par diverger (L137). Les
deux essais spécifiques sont remplacés par celui-ci.

```
RadialImpact<64> : energie_limite=1,485427e3  pente=0,448799  stokes=0,448799
ImpactField      : energie_limite=5,720523e3  pente=0,448737  stokes=0,448799
```

Ses deux assertions ne disent pas la même chose. **Ne pas dépasser** est la sûreté. **Atteindre**
est le contrat : si le champ limite reste loin sous la cambrure, c'est que la frontière borne
autre chose qu'une pente réelle.

### Garde 2 — `no_undeclared_comparison_to_max_slope_s143`

Elle lit les sources du crate et compare l'ensemble des comparaisons à `max_slope` à une liste
déclarée — deux constructions qui convertissent leur borne L1, trois budgets qui somment des
grandeurs déjà converties. Elle ne juge aucun calcul : elle constate un site.

Elle voit aussi les fichiers que le compilateur ignore, un module non déclaré compris. Ce n'est
pas un défaut : un fichier de code mort finit par être déclaré, et il vaut mieux qu'il soit
signalé avant.

### La vérification : réintroduire les fautes et les voir attrapées

Une garde qu'on n'a pas vue échouer ne garde rien — c'est la moitié qu'ADR-082 exige de chaque nom
de refus. Les deux fautes ont été réintroduites dans une copie de travail, puis annulées.

**Faute 1, celle de S141** — rendre à `ImpactField::new` sa comparaison L1 :

| garde | ce qu'elle a dit |
|---|---|
| recensement | `impact_field.rs : slope>medium.max_slope` au lieu de `slope/SLOPE_L1_RATIO>medium.max_slope` |
| essai générique | `ImpactField : pente=0,263716` contre 0,448799 attendu |

0,448799 / 1,701591 = 0,263748 : **l'essai ne signale pas seulement l'échec, il donne le facteur
manquant**, et il l'aurait donné en S141.

**Faute 2, celle qu'A210 décrit** — un troisième champ, dans un fichier que rien ne déclare :

```
left: [… ("champ_fictif.rs", "slope>medium.max_slope"), …]
```

Attrapée le jour où le fichier est écrit, avant même d'être branché.
