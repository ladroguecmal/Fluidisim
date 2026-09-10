
# ADR-097 — Ce qui garde le contrat de pente

- **Statut : actée**, S143, 2026-09-10, autonomie technique S71.
- **Traite** [A210](../registres/ANGLES-MORTS.md), ouverte par S142.
- **Prolonge :** [ADR-094](ADR-094-d-ou-vient-la-limite-de-pente.md) (la limite est la pente
  réelle), [ADR-096](ADR-096-les-deux-champs-disent-la-meme-chose-de-max-slope.md) (les deux
  champs la disent pareil).
- **Ajoute l'invariant I-18.**
- **Mesure :** [CONTRAT-PENTE-S143](../validation/CONTRAT-PENTE-S143.md).

## Problème

Après S141 et S142, les cinq comparaisons à `max_slope` du crate portent toutes sur une pente
réelle. Mais **le contrat ne vit nulle part** : deux commentaires, deux essais homonymes, et deux
constantes qui ne se déduisent pas l'une de l'autre — 1,795071 pour la quadrature de Hankel,
1,701591 pour les 40 modes cartésiens.

Un troisième champ écrirait `if slope > medium.max_slope` sans que rien n'échoue, et c'est
exactement ce que les deux premiers ont fait pendant soixante sessions. Le défaut n'est pas dans
un calcul : il est dans le **dispositif**.

## Ce qui a été pesé, et pourquoi le plus solide en apparence est écarté

La question qui décide n'est pas « qu'est-ce qui est le plus propre » mais **« qu'est-ce qui aurait
arrêté la faute de S141 »** et **« qu'est-ce qui échouerait le jour où quelqu'un l'oublie »**.

**Le type porteur est écarté**, et ce n'était pas la préférence de départ. `Medium::max_slope:
RealSlope` garderait à la compilation, sans rien à inscrire. Mais **l'hôte doit pouvoir construire
un `RealSlope`**, puisque c'est lui qui fournit le milieu : le constructeur est public, et un
troisième champ écrira `RealSlope::new(slope)` — trois mots — pour faire compiler sa comparaison
fausse. La garde est une bosse, pas un mur. Coût mesuré : 36 constructions de `Medium`, cinq
comparaisons, quatre contrôles de validité, trois lectures dont un hachage, quatre mutations
d'essai — **une cinquantaine de sites et une API publique changée, pour une bosse**.

## Décision

**Deux gardes exécutables, et un invariant qui dit ce qu'elles protègent.**

**1. `every_field_places_its_limit_at_stokes_steepness_s143`** — ce que les champs *calculent*.
Dichotomie sur l'énergie jusqu'au dernier champ admis avec `max_slope = BREAKING_SLOPE`, puis
mesure de la pente réelle. Deux assertions de sens différent : **ne pas dépasser** est la sûreté,
**atteindre** est le contrat — un champ limite qui reste loin sous la cambrure signale une
frontière qui borne autre chose qu'une pente.

Écrit **une fois** pour tous les champs. Les deux essais que S141 et S142 avaient écrits chacun de
leur côté sont retirés, leur contenu étant intégralement repris : deux textes qui se ressemblent
finissent par diverger (**L137**).

**2. `no_undeclared_comparison_to_max_slope_s143`** — ce que le crate *contient*. Elle lit les
sources et compare l'ensemble des comparaisons à `max_slope` à une liste déclarée. Elle ne juge
aucun calcul ; elle constate un site. C'est la seule des voies envisagées qui attrape ce qu'A210
décrit vraiment : **une implémentation de plus qui ignore le contrat**. La ligne que son auteur
doit ajouter à la liste l'oblige à dire quel rapport il applique, donc à l'avoir mesuré.

**3. L'invariant I-18** dit *pourquoi* les deux gardes existent, ce qu'aucun code n'exprime. Il ne
tient pas tout seul, et l'énoncé le dit : `I-14` a tenu soixante sessions parce qu'un essai le
vérifiait, pas parce qu'il était écrit.

## La vérification, qui fait partie de la décision

Une garde qu'on n'a pas vue échouer ne garde rien — c'est la moitié qu'ADR-082 exige de chaque nom
de refus. Les deux fautes ont été réintroduites puis annulées :

| faute | ce que les gardes ont dit |
|---|---|
| la comparaison L1 rendue à `ImpactField` (**la faute de S141**) | recensement : la ligne exacte ; essai : `pente = 0,263716` contre 0,448799 — soit le facteur 1,701591 manquant, **donné par le message** |
| un troisième champ, dans un fichier que rien ne déclare (**la faute qu'A210 décrit**) | recensement : `champ_fictif.rs : slope>medium.max_slope`, attrapé avant même d'être branché |

## Ce que cette décision ne fait pas

**Elle ne rend pas la faute impossible**, et aucune des trois voies ne le pouvait. Elle la rend
**bruyante** : au premier `cargo test`, avec un message qui nomme le rapport manquant.

**Elle ne couvre que ce crate.** Le recensement lit `src/` de `water-core` ; un champ écrit
ailleurs y échapperait. Aujourd'hui aucun autre crate ne compare quoi que ce soit à `max_slope` —
si cela change, c'est le recensement qui doit s'étendre, et il vaut mieux que ce soit dit ici que
découvert plus tard.

**Elle n'ajoute pas de trait commun aux champs.** Les deux `sample` ont la même signature, ce qui
rendait un trait tentant ; il n'aurait eu qu'un seul usage — une surface publique sans lecteur,
exactement ce qu'ADR-082 refuse. Les deux champs sont passés à la garde par des fermetures.

## Réception

273 tests, cinq ignorés — le même compte qu'à l'entrée de la session : deux essais retirés, deux
gardes ajoutées. Aucun hachage touché, harnais H1 inchangé, aucun code de calcul modifié.
