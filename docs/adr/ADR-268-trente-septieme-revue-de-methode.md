# ADR-268 — Trente-septième revue de méthode (S661–S665)

- **Statut : actée**, S666, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-267](ADR-267-trente-sixieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S661 | la revue (ADR-267) | — | — |
| S662 | **deux bornes posées de tête** : `k` baisserait de moins de 10 % (10,9 % : l'assertion du script l'a arrêtée avant l'écriture) ; la limite linéaire à 10⁻⁹ (le terme d'ordre `ε` de la forme composite, pris pour `ε²` : 10⁻⁷ mesuré, la proportionnalité vérifiée ensuite) | un critère manqué par sa borne | **D1** |
| S663 | **une borne qui ignorait le plancher de l'instrument** : 10⁻¹² exigé d'un champ écrit en `f32` (10⁻⁷) | un critère manqué par sa borne | **D1** — ADR-236 D1 le disait (le plancher écrit à côté du seuil) ; non appliquée |
| S664 | **deux remèdes jugés dans un montage où une autre cause nommée était encore active** (les parois) : la correction par Snell, juste, a été **rejetée** parce qu'elle « empirait l'eau mince » — sous les parois | une session ; un bon remède écarté puis repris | **D2** |
| S665 | le témoin propre (les parois et la normalisation ôtées) a montré `K_r` exactement ; le remède rejugé, retenu ; deux versions divergentes essayées et rejetées sur preuve | — | la protection d'ADR-259 a servi, une fois appliquée dans l'ordre |

## 2. Décisions

**D1 — Une borne du plan est calculée, avec le plancher de son instrument.** La ligne « calcul » du bloc des contrôles écrit, pour chaque
borne :

- sa valeur, calculée par le script du plan et non posée de tête ;
- le plancher de l'instrument qui la lit : la précision d'écriture (`f32` : 10⁻⁷), l'ordre de la formule (`ε` ou `ε²`), le quantum.

Une borne sous le plancher est refusée par le script. ADR-236 D1 et ADR-253 D1 le disaient déjà pour les seuils et les phrases du plan ;
S662 et S663 montrent qu'une **tolérance d'essai** y échappait.

**D2 — Un remède essayé quand une autre cause nommée est encore active n'est pas jugé : il est suspendu.** On l'essaie seulement après le
témoin qui isole sa cause. S'il a été essayé avant, son résultat ne le rejette pas ; il se rejuge, une fois les autres causes ôtées. ADR-259
D1 disait « un témoin qui supprime une cause avant de nommer un remède » ; S664 l'a appliqué au diagnostic, mais pas au jugement des
remèdes.

## 3. La prochaine revue

S671.
