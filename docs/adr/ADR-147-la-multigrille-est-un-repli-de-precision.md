# ADR-147 — La multigrille est un repli de précision, pas le solveur ordinaire

- Statut : **actée**, S245, 2026-09-16 ; autonomie technique S71.
- Ferme **A275**, ouverte par [TOLERANCE-PRESSION-S239](../validation/TOLERANCE-PRESSION-S239.md) :
  « f32 ne tient pas la tolérance physique à 32 768 mailles ».
- N'amende ni [ADR-143](ADR-143-la-pression-f32-converge-a-sa-precision-representable.md) ni
  [ADR-144](ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) : les deux portes
  d'acceptation sont **inchangées**, et c'est le solveur qui a changé.
- Mesures et réception : [MULTIGRILLE-S245](../validation/MULTIGRILLE-S245.md).

## Constat

[COUT-DELTA-S244](../validation/COUT-DELTA-S244.md) avait désigné la multigrille comme le seul levier
dont le gain croît avec la taille : le coût d'une itération est stable en structure, c'est leur
**nombre** qui double à chaque raffinement — 30, 61, 114, 220, 425 de 128 à 32 768 mailles.

Construite et mesurée, elle donne deux résultats opposés.

1. **En vitesse, elle perd.** Un cycle coûte cinq produits fins par itération là où le gradient
   conjugué nu en fait un. Les itérations tombent à 134 sur 425 à la plus grande taille, mais
   **triplent** aux petites, où la hiérarchie n'a qu'un ou deux niveaux. Le pas est plus long partout.
   Contre-épreuve faite avant de conclure : une hiérarchie plus profonde et un cycle allégé
   **aggravent** — 282 à 300 itérations, plates mais hautes. Un compte plat et haut accuse le
   **transfert**, pas le niveau grossier ni le lisseur.
2. **En précision, elle gagne, et c'est A275.** À 32 768 mailles, la divergence projetée tombe de
   **1,339·10⁻⁵ — refusée par ADR-144 — à 8,512·10⁻⁶, reçue**. La cause de A275 n'était pas la taille
   en soi : c'était **l'arrondi accumulé sur 425 itérations**. En en demandant 134, le solveur
   accumule moins, et le **même** test, appliqué au **même** vrai résidu recalculé, passe.

## Décision

1. **La multigrille est un repli, pas le solveur ordinaire.** Le pas s'exécute d'abord par le chemin
   de S244 ; **s'il est refusé**, et seulement alors, il est rejoué avec le préconditionneur
   multigrille. On ne paie le cycle que là où l'autre échoue.
2. **Aucune porte d'acceptation ne bouge.** Un préconditionneur change les directions de recherche,
   jamais le verdict : le résidu premier de 10⁻⁶ et la tolérance physique restent ceux d'ADR-144.
   **A275 est fermée parce que le solveur y satisfait, pas parce que le seuil aurait été relâché.**
3. **Le préconditionneur doit être symétrique défini positif**, sans quoi la récurrence du gradient
   conjugué n'est plus valide. Trois choses l'assurent et aucune n'est décorative : lisseur
   **diagonal** (son propre adjoint), restriction **transposée** de la prolongation, **autant de
   lissages après qu'avant**. La propriété est **vérifiée numériquement**, jamais supposée.
4. **L'opérateur des niveaux grossiers est re-discrétisé**, ouvertures et fractions moyennées. C'est
   une approximation, et elle est licite : elle ne pèse que sur la vitesse de convergence.
5. **Le mode à surface mobile ne l'a pas.** Il garde son Jacobi diagonal ; le grossissement de ses
   lignes à fantôme n'est pas écrit.
6. Nombres de lissages et règle d'arrêt de la hiérarchie sont des **données de coût**, au même titre
   que le grain d'une tâche (ADR-029 §3) : elles changent la vitesse, jamais le verdict.

## Conséquences

- **δ accepte 32 768 mailles**, ce qu'il refusait depuis S239. Le pas y coûte 1 006 ms au lieu de
  319 refusées — le premier essai perdu plus le cycle.
- **Rien ne change aux tailles qui passaient déjà**, au bit : mêmes itérations, mêmes résidus, même
  empreinte `delta_filters` `0xfb12b2092df4ee6d`. Le repli ne se déclenche pas.
- **Le coût de δ reste entier** : 143 fois le budget d'ADR-125 à 32 768 mailles. Ce lot n'a pas
  gagné de vitesse, et il ne prétend pas le contraire.
- **La suite est nommée et mesurée** : la prolongation constante par morceaux est ce qui plafonne le
  taux par cycle. Une prolongation bilinéaire, avec sa restriction transposée, est le prochain pas —
  les essais d'adjonction et de symétrie sont déjà écrits pour l'accueillir.
- Une machine, un pas de temps, un fond plat au banc de coût.

## Réversibilité

Retirer le repli — une condition dans `run` — rend exactement le chemin de S244, au bit, et rouvre
A275. La hiérarchie resterait construite et testée, sans consommateur.

**Correction factuelle S252, 2026-09-16.** Le gradient conjugué multigrille mesuré ici avait un β
fautif (`‖r_{n+1}‖²/⟨r_n, z_n⟩`, A285). Le constat 1 (« en vitesse, elle perd ») et les comptes
d'itérations sont donc invalides. Corrigée, la multigrille converge en 6 à 8 itérations et est
plus rapide dès 512 mailles, 2,9 fois à 32 768. Le constat 2 est mal expliqué : la tolérance
tenait grâce aux relances du gradient fautif, pas parce qu'il faisait moins d'itérations.
Corrigée, elle s'arrête au plancher à 32 768 mailles (`D = 1,585·10⁻⁵`) ; le pas y est reçu par
l'affinage d'[ADR-151](ADR-151-affinage-au-pas-fixe-et-travail-compte.md). La décision 1 reste
appliquée tant qu'un nouvel ADR ne l'a pas remplacée ; sa prémisse de vitesse est réfutée.
[Mesures](../validation/MULTIGRILLE-BETA-S252.md).
