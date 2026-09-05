# ADR-020 — Le système d'eau est une bibliothèque sans dépendance moteur

- **Statut** : proposée — **contrainte bloquante, à acter avant la première ligne de code**
- **Session** : S03
- **Dépend de** : ADR-002, ADR-007, ADR-012
- **Conditionne** : `SPEC-003` (harnais de validation) et les onze bancs

---

## 1. Le raisonnement, à l'envers

Cet ADR ne part pas d'un besoin d'architecture. Il part d'une contrainte du harnais de validation,
et remonte jusqu'à la structure du code.

Le plan de benchmark exige onze bancs, dont plusieurs demandent des centaines d'exécutions
(convergence sous raffinement, iso-qualité entre solveurs candidats, bisection sur régression).
Si chaque exécution demande de démarrer le moteur, de charger un niveau et d'attendre la
compilation des shaders, alors :

- une campagne de convergence coûte des heures au lieu de minutes ;
- la validation ne tourne pas à chaque commit, donc les régressions sont trouvées tard ;
- le harnais devient un projet à part entière, puis il est abandonné.

C'est le scénario habituel. **Le harnais n'échoue pas parce qu'il est mal écrit ; il échoue parce
que le système testé ne se laisse pas instancier seul.** Et cela se décide au premier jour, pas au
moment d'écrire les tests.

> **Décision : le système d'eau est une bibliothèque autonome. Le moteur est un hôte parmi
> d'autres ; le harnais en est un second.**

---

## 2. Ce que la bibliothèque n'a pas le droit de connaître

| Interdit | Fourni à la place |
|---|---|
| L'horloge du moteur | `T_sim` en entrée (ADR-003) |
| Le graphe de scène | une liste de `SolidProxy` poussée par l'hôte |
| Le système de fichiers, le streaming d'assets | `IBathymetryProvider`, `IHydroSampleProvider` |
| Le renderer, ses ressources GPU | l'hôte fournit un `IGpuBackend` minimal, ou aucun |
| Le système de tâches du moteur | `IJobSystem` injecté, avec une implémentation triviale mono-fil |
| La journalisation, la télémétrie | `ISink` injecté |
| Toute allocation globale | un allocateur fourni au démarrage (I-06) |

Ces interfaces sont l'intégralité de la surface d'hébergement. Elles doivent tenir sur une page ;
si elles grossissent, c'est que du moteur est en train de rentrer.

> **Précision S04.** Le décompte exact est de **six interfaces et un paramètre** : l'horloge n'est
> pas un service mais une valeur poussée à `begin_tick`. C'est plus strict que ce qui était prévu
> ici — un système incapable de lire l'heure ne peut pas en dépendre par accident. Détail en
> [`specs/SPEC-004-interfaces.md`](../specs/SPEC-004-interfaces.md) §8.

## 3. Trois hôtes, un seul cœur

```
                  ┌──────────────────────────────┐
                  │   cœur : B · W · δ · V       │
                  │   ordonnanceur, budgets      │
                  └──────────────┬───────────────┘
                                 │  sept interfaces
        ┌────────────────┬───────┴────────┬──────────────────┐
   hôte moteur      hôte harnais      hôte serveur      (hôte outil)
   rendu réel       sans interface    sans rendu ni δ    éditeur, précalcul
```

L'**hôte serveur** est le contrôle le plus utile : il n'exécute ni δ ni rendu (ADR-009 §4). S'il
compile et tourne, c'est la preuve mécanique que l'autorité gameplay ne dépend pas du
volumétrique. L'invariant I-04 cesse d'être une règle de revue de code pour devenir une propriété
vérifiée par le build.

## 4. Contreparties assumées

- **Une couche d'indirection sur le chemin chaud.** Bornée : les interfaces sont appelées par
  lot (une liste de proxys par tick, pas un appel virtuel par cellule). Aucune indirection à
  l'intérieur d'un solveur.
- **Deux implémentations à maintenir** pour chaque interface (moteur + harnais). L'implémentation
  harnais est triviale par construction ; si elle ne l'est pas, l'interface est mal découpée —
  c'est un signal utile.
- **Discipline permanente.** Une dépendance moteur se réintroduit en une ligne. Elle doit être
  interdite par le build, pas par la revue : le cœur compile dans une cible qui n'a pas accès aux
  en-têtes du moteur. Test automatique, pas bonne volonté.

## 5. Bénéfices non recherchés

Découverts en écrivant cet ADR, ils dépassent le motif initial :

- **Le précalcul côtier** (ADR-013 §4) et la **gravure de terrain d'après les rivières**
  (ADR-011 §3.1) sont des outils hors ligne. Ils réutilisent le cœur tel quel au lieu d'en
  réimplémenter une variante — cause classique de divergence entre l'outil et le jeu.
- **Portage.** Changer de moteur, ou en cibler deux, devient un travail d'hôte.
- **Bisection automatique** sur régression (SPEC-003 §6) : possible uniquement parce qu'une
  exécution coûte des secondes.

## 6. Ce qui reste ouvert

1. Langage et cible du cœur — décision de l'équipe technique, sans incidence sur cet ADR.
2. `IGpuBackend` : quelle surface minimale pour permettre un solveur δ sur GPU sans importer
   l'abstraction graphique du moteur ? C'est l'interface la plus risquée des sept ; à prototyper
   avant B3.
3. Le harnais doit-il pouvoir héberger un **rendu de référence** minimal pour les comparaisons
   perceptuelles, ou celles-ci passent-elles par l'hôte moteur ? Voir `SPEC-003` §5.
