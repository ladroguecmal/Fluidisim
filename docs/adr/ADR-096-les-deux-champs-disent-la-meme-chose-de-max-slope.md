
# ADR-096 — Les deux champs disent la même chose de `max_slope`

- **Statut : actée**, S142, 2026-09-10, autonomie technique S71.
- **Traite** [A209](../registres/ANGLES-MORTS.md), ouverte par la migration de S141.
- **Prolonge :** [ADR-094](ADR-094-d-ou-vient-la-limite-de-pente.md) (la limite est la pente
  réelle), ADR-081 (cohérence de vocabulaire entre constructeurs).
- **Revient sur** la dispense d'ADR-082 §65, dont le motif a cessé de valoir.
- **Mesure :** [PENTE-MODALE-S142](../validation/PENTE-MODALE-S142.md).

## Problème

S141 a fait de `max_slope` la borne de la pente **réelle** pour `RadialImpact`. `ImpactField` — le
champ modal à 40 modes d'ADR-058 — lui comparait toujours sa **borne L1**. Un même `Medium` passé
aux deux constructeurs produisait donc deux frontières de sens différent, sans que rien dans le
type ne le signale. **Le défaut a été introduit par la session précédente, pas par le code
d'origine**, et il a été nommé plutôt que déplacé.

Trois issues étaient ouvertes : mesurer et migrer, séparer les deux sens dans le type, ou retirer
un champ que plus aucun appelant de production ne construit.

## Ce qui a été mesuré

**Rapport borne L1 / pente réelle = 1,701591**, et c'est **une constante du modèle** — invariante
sur `λ` de 0,5 à 32 m et `E` de 1e-4 à 10 J, stable dès 25 points de grille par côté, maximum
atteint à `t = birth` en `[0 ; 0,0733]·side`.

Deux raisons structurelles, et elles se vérifient : `side = 4λ` avec des modes indexés par des
entiers rend le motif identique à toute longueur d'onde ; le champ est périodique et `sample`
n'impose aucune emprise, donc **le maximum est toujours atteint**. C'est ce second point qui
sépare ce cas de celui de la pression (A206), où une emprise étroite pouvait manquer le maximum et
faire diverger le rapport.

La borne a été retrouvée **par dichotomie sur `max_slope`**, sans rien changer à la bibliothèque —
c'est-à-dire telle que l'extérieur la voit, qui est exactement la grandeur qu'A209 met en cause.

## Décision

**1. `ImpactField::new` compare la pente réelle**, comme `RadialImpact::new` depuis S141 :
`slope / SLOPE_L1_RATIO > medium.max_slope` rend `Steepness`. `max_slope` a désormais **un seul
sens** dans ce crate, et `BREAKING_SLOPE` est la valeur qui le remplit pour les deux.

**2. La constante est publiée sous le même nom que celle du candidat radial**, dans son propre
module : `impact_field::SLOPE_L1_RATIO = 1,701591` en regard de
`radial_impact::SLOPE_L1_RATIO = 1,795071`. L'homonymie est **voulue** — même grandeur, autre
champ, autre valeur — et le chemin de module dit lequel. C'est l'application directe d'ADR-081 :
*deux constructeurs du même crate ne doivent pas nommer différemment la même distinction.*

**3. Le champ n'est pas retiré, et ADR-059 est la raison.** Il y est conservé comme « support de
comparaison physique » : il a été écarté du chemin actif, ce qui n'est pas la même chose
qu'abandonné. « Personne ne le construit » n'établit pas qu'il est mort — le dépôt a payé cette
confusion une fois, au troisième fork, sur une branche gardée exprès que rien ne distinguait d'un
point de départ (S39).

## Pourquoi la dispense d'ADR-082 ne vaut plus

ADR-082 §65 avait délibérément laissé ce champ de côté : « ses bornes ont la même maladie, mais il
n'est plus le chemin actif, et le renommer sans consommateur ajouterait du travail sans lecteur ».

Ce motif était juste **tant que les deux constructeurs disaient la même chose**. Depuis S141 ils ne
la disent plus, et le lecteur qui manquait existe : c'est quiconque lit `Medium::max_slope`. Une
dispense accordée pour absence de conséquence tombe le jour où la conséquence apparaît — elle n'a
pas à être révoquée, elle cesse simplement de s'appliquer.

## Ce que cette décision ne fait pas

**Elle ne remet pas `ImpactField` sur le chemin actif.** ADR-059 tient : il ne doit pas être
raccordé tel quel comme impact régional, et rien ici ne le raccorde.

**Elle n'ajoute pas d'accesseurs sans lecteur.** `slope_bound()` et `slope_max()` existent sur
`RadialImpact` parce que le budget de composition les consomme ; ce champ-ci n'entre dans aucun
budget, et lui greffer la même surface serait exactement le travail sans lecteur qu'ADR-082
refusait à raison. L'essai de réception mesure la frontière **de l'extérieur**, par dichotomie —
il n'a besoin d'aucun accesseur, et il vaut pour cela mieux qu'un accesseur.

**Elle ne corrige pas les autres noms de ce module.** ADR-082 notait que sa condition de régime est
nommée `Medium`, ce qui est faux de la même façon ; cela reste vrai et reste noté.

## Réception

273 tests, cinq ignorés — un de plus.
`the_admitted_limit_field_sits_exactly_at_stokes_steepness_s142` reprend mot pour mot l'essai que
S141 a écrit pour le candidat radial : dichotomie sur l'énergie jusqu'au dernier champ admis avec
`max_slope = BREAKING_SLOPE`, puis mesure de la pente réelle sur une période spatiale complète.

```
energie_limite = 5,720523e3 J    pente = 0,448737    stokes = 0,448799
```

**Écart relatif 1,4e-4.** Les deux champs de ce crate placent maintenant leur champ limite à la
cambrure limite de Stokes, et c'est un essai qui l'atteste, pas un commentaire.

Aucun hachage de campagne touché, harnais H1 inchangé : ce champ n'a aucun consommateur de
production, ce qui rend cette migration-ci gratuite — contrairement à celle de S141.
