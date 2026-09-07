# Audit des garde-fous — S34

Revue des **dix garde-fous** du cœur et du harnais, selon un critère unique : *ce garde-fou a-t-il
déjà été vu refuser ?*

**Origine.** Angle mort **A144**, trouvé en S33. Un contrôle de réflexion vérifiait qu'on mesurait
**derrière le front** et non que le front **n'avait jamais atteint le mur**. L'idée était juste, la
condition non — et le contrôle déclarait saine une fenêtre polluée par une onde réfléchie.

> **Un garde-fou en qui l'on a confiance est plus dangereux qu'aucun garde-fou.** Un montage sans
> contrôle est réexaminé à chaque usage ; un montage qui en a un ne l'est plus.

---

## 1. Le critère

> **Un garde-fou qu'on n'a jamais vu déclencher n'a pas été testé.**
>
> Le test d'un garde-fou est **le cas qu'il doit refuser**, jamais le cas nominal — celui-ci passe
> de toute façon, et son succès ne dit rien.

**Corollaire, apparu en cours d'audit** : un garde-fou doit aussi porter son **témoin**, c'est-à-dire
le cas sain qu'il ne doit **pas** refuser. Sans lui, un contrôle qui refuserait tout passerait son
propre test. Trois des dix en ont un ; les autres n'en avaient pas besoin, leur cas nominal étant
exercé par ailleurs.

## 2. L'inventaire, et le cas que chacun doit refuser

| | Garde-fou | Cas qu'il doit refuser | Verdict |
|---|---|---|---|
| **G1** | pas de temps sur domaine sec (`dt_cfl`) | aucune cellule ne porte d'eau — `vmax = 0` | refuse : pas de repli fini et positif |
| **G2** | bornage de `ν` (`avec_cfl`) | **que `ν = 1,5` soit ramené sous 1** | laisse passer — corrigé en S29, vérifié ici |
| **G3** | plancher d'arrondi (`Convergence::ordre`) | trois erreurs entièrement sous le plancher | refuse : `Plancher` |
| **G4** | longueur de série (`ordre_final`, `asymptotique`) | deux points pour une extrapolation qui en demande trois | refuse : `Indetermine`, puis `None` |
| **G5** | amplitude de seiche (`mesurer_seiche`) | `a` sous l'ulp du `f32` — aucun extremum détectable | refuse : `None` |
| **G6** | réflexion (`c33_decroissance_entretenue`) | le montage à `R² = 0,487` de S33 | refuse — **et laisse passer le domaine long** |
| **G7** | seuil de front (`front`) | un seuil qu'aucune cellule n'atteint | refuse : `None` |
| **G8** | référence nulle (`Cas::ecart_rel`) | la division par zéro que C01 impose | refuse : bascule sur l'écart absolu |
| **G9** | définition d'`u_max` | confondre absolue et gouvernante | distingue — **et coïncide sans paroi** |
| **G10** | bornage de l'ordre grossier | un ordre **négatif**, signature du pré-asymptotique | **masquait** — voir §3 |

**Neuf sur dix refusent correctement.** Chacun a désormais son test de déclenchement, dans
`tests_garde_fous` et `tests_g10`.

## 3. G10 masquait, et il était le seul sans test

Le bornage `clamp(0,3 ; 3,0)` sur l'ordre estimé aux trois grilles les plus grossières — utilisé
pour estimer l'erreur de l'oracle, elle-même utilisée pour filtrer les grilles contaminées —
corrigeait **en silence**.

**Or S24 a mesuré des ordres négatifs** : −0,504 puis −0,059 sur le front de C04, signature du régime
pré-asymptotique où les différences successives *grandissent*. Un ordre hors bornes n'est donc pas
une valeur à corriger :

> **C'est le signe que les grilles grossières ne sont pas en régime asymptotique**, et que
> l'estimation d'erreur d'oracle qui en dépend n'a **aucun fondement**. Le borner revient à répondre
> à une question dont on vient d'apprendre qu'elle n'a pas de réponse.

### 3.1 La correction, en trois gestes

1. **Le bornage reste.** Il faut un nombre pour filtrer, et il est **conservateur** : un `p` bas
   surestime l'erreur d'oracle, donc écarte *plus* de grilles. Le supprimer rendrait le filtre
   erratique.
2. **Il est signalé** — au `Sink`, et dans le libellé de la grandeur : « ORDRE GROSSIER HORS BORNES,
   filtre indicatif ». Un garde-fou qui corrige sans le dire transforme une anomalie en résultat.
3. **L'estimation est extraite** en fonction pure `ordre_grossier_estime(erreurs) → (brut, borné)`.

### 3.2 Le troisième geste est le plus important

L'estimation vivait **en ligne** dans une fonction qui lance des simulations : la vérifier demandait
d'en exécuter une, avec un oracle à 51 200 cellules.

> **Un garde-fou qu'on ne peut pas exercer isolément est un garde-fou qu'on n'exercera pas.**

G10 était le **seul des dix sans test**, et le **seul défaillant**. Ce n'est probablement pas une
coïncidence : les neuf autres étaient appelables directement, donc ont été éprouvés au fil des
sessions ; celui-là ne l'était pas, et personne ne l'a regardé pendant dix sessions.

## 4. Ce que l'audit dit de la méthode

**Le critère « l'a-t-on vu refuser ? » est plus discriminant que la relecture.** Les dix garde-fous
ont tous été relus plusieurs fois au fil des sessions, et G10 a survécu à ces relectures parce qu'il
*a l'air correct* : borner une estimation entre deux valeurs raisonnables est un geste ordinaire, et
rien dans sa formulation ne dit qu'il masque.

**Ce qui l'a désigné n'est pas sa forme, c'est son absence de test** — et l'absence de test venait
de sa non-testabilité, qui venait de son emplacement. La chaîne est mécanique :

> emplacement en ligne → non testable isolément → jamais testé → jamais vu refuser → défaut invisible

**Réflexe à porter au harnais** : tout garde-fou nouveau s'écrit comme une fonction **appelable
seule**, et son premier usage est son test de déclenchement. Le coût est de quelques lignes ; le gain
est qu'il existera un endroit où l'on peut lui poser la question.

## 5. Ce qui reste ouvert

1. **Les `max(0.0)` de la physique n'ont pas été audités.** Ils sont une douzaine dans `delta.rs`
   (hauteur négative, reconstruction hydrostatique, état initial) et relèvent d'une autre famille :
   ce sont des **saturations de modèle**, pas des contrôles de validité de montage. Une saturation
   qui se déclenche souvent signale un problème ; aucune n'est comptée. Angle mort **A146**.
2. **Aucun garde-fou ne compte ses déclenchements.** On sait qu'ils *peuvent* refuser ; on ne sait
   pas s'ils refusent **en usage réel**, ni à quelle fréquence. Un compteur par garde-fou coûterait
   peu et dirait lesquels travaillent.
3. **Le signalement de G10 passe par le `Sink` et le libellé**, qui ne sont lus par personne
   automatiquement. Il faudrait qu'un déclenchement se voie dans le **verdict**, comme les témoins
   de C01 et C04.

## 6. Complément S54 — l'entrée vide

Les dix contrôles ont été confrontés à leur absence pertinente, avec témoin :
[GARDE-FOUS-VIDE-S54](../validation/GARDE-FOUS-VIDE-S54.md). Six tests complémentaires,
aucun faux succès supplémentaire trouvé sur ces entrées. G1 couvrait déjà le domaine sec.
Une fenêtre sans réflexion ne prouve pas la présence de l'onde : batteur arrêté, le montage
C33 refuse par absence de profil. S43-2 close ; refus du premier témoin G5 suivi en S54-1.
