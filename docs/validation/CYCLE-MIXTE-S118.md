# S118 — Cycle hôte temporel mixte piloté par le contrôleur

2026-09-09. S117-1 réalisée. Aucune décision nouvelle : ADR-078 est exercée, pas modifiée.

## Ce que le cycle exerce

`code/water-core/examples/cycle_mixed.rs` remplace la préparation directe par le contrôleur
dans la boucle de l'hôte, aux deux recettes que S116 a reçues spatialement — 224×128 et
256×128. La séquence d'instants n'est pas monotone :

```
0 · 499 999 · 500 000 · 500 001 · 1 500 000 · 1 500 000 · 2 000 000
  · 2 500 000 · 4 000 000 · 0 · 1 500 000 · 500 000        (µs)
```

Elle contient une avance, deux pas d'une microseconde, une répétition, un retour à zéro et
un retour arrière de 1 s. À chaque étape, dans cet ordre : `state` est comparé à l'état
attendu, `current` est refusé si la date demandée n'est pas publiée, `update` est appelé
puis rappelé au même instant, et la requête mixte B + impact + deux pressions est évaluée
sur 289 points par la vue du contrôleur, puis par une préparation directe du même journal
sur un troisième pool.

**Les deux voies coïncident en bits à chaque étape** — les dix composantes de chaque
échantillon, plus énergie, puissance, enveloppe de pente et instant publié. 3468 points-temps
par recette. Hachages `6591ab360344f76e` (224×128) et `b563610d1dd78ada` (256×128), stables
d'une exécution à l'autre. Deux `Unchanged` sont attendus et comptés : la construction publie
déjà l'instant 0, et le cycle répète 1,5 s.

## Trois refus reçus

1. **Hors fenêtre de pression** (8 000 001 µs) : `update` refuse, `state` rend
   `OutsideWindow`, la date publiée ne bouge pas et l'énergie de la vue reste identique en bits.
2. **Point hors domaine** : la requête mixte refuse et la sortie précédente n'est pas touchée,
   pour deux points hors des bornes déclarées.
3. **Publication au-delà de la validité des impacts** — le fait neuf, plus bas.

## Deux horizons distincts, et le contrôleur n'en connaît qu'un

La fenêtre de publication de la pression va jusqu'à 8 s ; les impacts du montage expirent à
4 s. **`update(6 s)` réussit** : le contrôleur valide la fenêtre de son contexte, qui ne dit
rien des impacts. La vue produite est finie et exploitable pour la pression seule. C'est la
**requête mixte** qui refuse, sur `renewal_deadline`, et l'hôte repart ensuite normalement
en publiant à 1,5 s.

Ce n'est pas un défaut du contrôleur : c'est la conséquence de ce qu'il tient. Mais un hôte
qui lit « publication réussie » et en conclut « je peux échantillonner » se trompe, et rien
dans le type ne l'en avertit. Le cas est verrouillé par un test de bibliothèque —
`controller_publishes_beyond_impact_validity_and_query_refuses`, recette 16×24 — et enregistré
comme **A194**.

## Coûts, et ce que la mesure a d'abord raconté de faux

Médianes sur 21 mesures, trois échauffements, Windows, machine non isolée, cuisson du spectre
et admission exclues.

| | 224×128 | 256×128 |
|---|---|---|
| `update` vers un instant qui change | 16,0370 ms | 14,3016 ms |
| **le même, mesuré en dernier** | **12,5520 ms** | **15,0662 ms** |
| `update` au même instant (`Unchanged`) | 0,1 µs | 0,1 µs |
| requête mixte 64 points sur la vue | 35,5694 ms | 40,0365 ms |
| `update` + `current` + requête 64 | 48,8190 ms | 56,0587 ms |
| préparation directe, témoin | 12,6307 ms | 14,5624 ms |
| directe, instant alterné | 12,3769 ms | 14,3276 ms |
| directe, pool alterné | 12,4002 ms | 15,3298 ms |

**La première lecture était fausse.** `update` semblait coûter 15 à 28 % de plus que la voie
directe à 224×128, et l'écart était instable d'une exécution à l'autre. Deux explications se
présentaient — le contrôleur alterne deux pools, donc double son empreinte mémoire ; et la
mesure alterne l'instant pour forcer le recalcul, ce que le témoin direct ne faisait pas.
**Les deux sont fausses** : la voie directe soumise à l'instant alterné, puis à deux pools
alternés, reste au niveau de sa mesure nominale.

Le troisième témoin tranche : le **même appel `update`, mesuré en dernier au lieu du premier**,
rejoint la voie directe (12,55 contre 12,63 ms). Sur quatre exécutions, l'écart apparaît trois
fois à 224×128 — la première recette du processus — et jamais à 256×128, où les deux mesures
du même code coïncident à 1 % près. **C'est la position dans la séquence de mesure qui parle,
pas le contrôleur.** Les trois échauffements de `measure` ne suffisent pas à mettre cette
machine en régime. Voir **A195** et **L207**.

### Ce que les chiffres disent une fois corrigés

- **Le contrôleur ne coûte rien de plus que la voie directe.** Il ne recopie pas les
  coefficients ; l'échange de pools est un `swap` de références.
- **Le chemin `Unchanged` coûte 0,1 µs**, soit une comparaison d'instants. C'est le gain
  réel et chiffré du contrôleur : une requête supplémentaire au même instant économise la
  préparation entière, 12,6 ms à 224×128.
- **La requête sur 64 points coûte près de trois fois la préparation** — 35,6 ms contre
  12,6 ms, soit environ 556 µs par point à 224×128. Le contrôleur ne change rien à ce
  rapport, et le cycle complet reste à ~49 et ~56 ms. **Aucun budget de trame n'est
  approché**, ce que S107 disait déjà et que S118 ne corrige pas.

## Ce qui n'est pas revendiqué

Aucune précision spatiale nouvelle : les références f64 de S116 ne sont pas relancées, la
campagne compare deux voies du même candidat entre elles. Aucun changement de bibliothèque
hors le test ajouté et la fixture de test scindée. L'admission reste figée pendant la vie du
contrôleur. Les mesures viennent d'une machine unique, non isolée, et ne certifient rien.

## Suite

**S118-1, S119 :** faire de la validité des impacts une donnée que le contrôleur puisse
consulter, ou publier explicitement l'horizon effectif du montage — A194. Restent ouverts :
admission dynamique dans le contrôleur, renouvellement de fenêtre, profondeur finie de
pression (S116-2), bilan mixte, durabilité disque.

78 ADR, 195 angles, 17 invariants, 6 spécifications, 23 cas.
