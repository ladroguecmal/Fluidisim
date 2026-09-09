# ADR-078 — Contrôleur de publication du champ de pression

- **Statut : actée**, S117,2026-09-09, autonomie technique S71.
- **Prolonge :** enveloppe S105, journal ADR-075, requête mixte ADR-077.
- **Résout :** S116-1, publication temporelle sur journal figé.

## Problème

La pression est préparée pour un instant exact. L'hôte assemblait lui-même deux pools,
la tentative de préparation et la bascule. Conserver un champ après une erreur ne suffit
pas : le servir comme s'il représentait le nouvel instant produirait une réponse périmée.

## Décision

`bound_pressure::Controller` emprunte un contexte, un demi-spectre contrôlé, un journal
immuable et deux pools de coefficients exclusifs. Les capacités des deux pools sont
vérifiées avant toute écriture initiale. Le constructeur prépare la première publication ;
un constructeur refusé peut avoir modifié son pool candidat, mais ne retourne aucune vue.

`update(time)` prépare uniquement dans le pool de réserve. Après calcul des coefficients,
énergie, puissance et enveloppe de pente, et après tous les contrôles, il échange les deux
pools et publie ensemble les métadonnées et l'instant. Aucune copie des coefficients,
allocation ni opération faillible après le début de la bascule. En cas de refus, le pool
de réserve peut être modifié ; le pool actif, ses bilans et son instant sont conservés.

L'instant déjà publié rend `Unchanged`, sans recalcul. C'est correct parce que le journal,
la recette et le contexte sont figés par emprunt pendant la vie du contrôleur. Les autres
instants de la fenêtre sont recalculés absolument depuis les sources : retour temporel
autorisé, sans intégrer un état précédent. Le contrôleur ne prolonge pas la fenêtre.

`state(requested)` distingue `Ready`, `NeedsUpdate { published }` et
`OutsideWindow { published }`. `current(requested)` retourne une vue uniquement si
l'instant correspond exactement à celui publié ; sinon `Error::Time`.
Après un échec, l'hôte peut demander explicitement l'ancien instant, jamais obtenir ce
champ sous une autre date. La vue emprunte le contrôleur et empêche une mise à jour
pendant son utilisation. Elle alimente les requêtes locales, monde et mixte existantes.

Les métadonnées spectrales validées sont conservées séparément des slices pour éviter
une structure auto-référente. `FieldState` est opaque hors du crate et n'est associé
qu'au préfixe du pool effectivement publié ; aucun constructeur public de champ à partir
de coefficients arbitraires n'est ajouté. Aucun `unsafe`, aucun format sérialisé nouveau.

## Portée et limites

Ce contrôleur gère un **journal figé**, pas l'admission de nouvelles sources. Pour changer
le journal, il faut libérer le contrôleur ; une future transaction d'admission ne devra
pas présenter l'ancien champ comme représentant le journal modifié. Un journal en attente
reste refusé à la construction. Aucun abandon de source ni renouvellement automatique.

La réception numérique et physique des champs reste celle du noyau existant. La bascule
est atomique au sens de la publication logique sur emprunts Rust, pas un protocole de
mémoire partagée multithread ou de persistance disque. Aucun coût nouveau certifié.

## Réception

[CONTROLEUR-PRESSION-S117](../validation/CONTROLEUR-PRESSION-S117.md) : comparaison directe
sur dix dates, deux pools alternés, retour temporel, refus numérique répété, capacités,
journal bloqué et requête mixte après mise à jour et refus. Les bilans et les sept sorties
de pression sont comparés en bits. Suite S117-1 : cycle temporel aux résolutions reçues.
