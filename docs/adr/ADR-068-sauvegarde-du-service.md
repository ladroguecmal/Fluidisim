# ADR-068 — Sauvegarder le service publié et son attente

- **Statut : ACTÉE**, S87, 2026-09-08, délégation technique.
- Complète ADR-057 et ADR-067 ; WJNL V1 et Impact V1 inchangés.

## Décision

`LiveWater::save` écrit une enveloppe **WLIV V1**, suivie du journal publié WJNL.
Elle contient le contexte de reconstruction, N, l'horizon depuis la naissance et la commande
en attente éventuelle. Les champs calculés et les réserves partiellement construites ne sont
pas sauvegardés. Le blocage est un état du service, même si le journal publié n'a aucune perte.

Toutes les valeurs sont little-endian ; les flottants utilisent leurs bits f32. En-tête fixe :

| Octets | Contenu |
|---|---|
| 0–3 | WLIV |
| 4–5 | version u16 = 1 |
| 6–7 | réservés nuls |
| 8–11 | N, u32 |
| 12–15 | FrameId, u32 |
| 16–23 | cellule, u64 |
| 24–43 | gravité, densité, profondeur, pente maximale, rayon : cinq f32 |
| 44–47 | réservés nuls |
| 48–55 | âge numérique publié, u64 |
| 56 | attente : 0 aucune, 1 prédiction, 2 confirmation, 3 rejet |
| 57–64 | époque de la commande, u64 |
| 65–84 | cause : entité u64, commande u64, émission u32 |
| 85–160 | Impact V1, 76 octets ; nuls pour rejet/absence |
| 161–167 | réservés nuls |
| 168… | WJNL V1 complet, longueur exacte vérifiée par son codec |

Sans attente, les octets 57–160 sont nuls. Taille totale : **192 + 97 × nombre
d'enregistrements publiés**. Le tampon de sortie est fourni par l'hôte ; une capacité
insuffisante est refusée avant écriture. La queue au-delà de la taille utilisée reste intacte.

## Restauration construite

`restore` travaille dans le journal et les champs de réserve, avec un tableau temporaire de
Record fourni par l'hôte. N, référentiel, cellule, milieu et rayon doivent correspondre bit
à bit à ceux du service cible. L'âge sauvegardé est restauré ; zéro est refusé. La même époque
serveur est requise par WJNL. Aucun changement de géométrie ni de résolution n'est implicitement reçu.

Le codec valide la commande en attente, y compris sa transition contre le journal restauré.
Un manque de capacité lors de cette vérification est autorisé : c'est précisément un motif
possible d'attente. Époque, autorité ou conflit invalides sont refusés. Cette vérification
n'applique pas la commande au journal publié.

Les champs publiés sont reconstruits depuis leurs événements. Seulement après succès complet,
journaux et pools sont échangés et l'attente restaurée. Un refus laisse le journal, les champs,
le contexte et l'attente actifs intacts ; réserves et temporaire peuvent avoir changé.
La restauration ne promet pas de couvrir l'instant réel courant : l'horizon enregistré est
conservé, puis l'hôte renouvelle explicitement si nécessaire.

## Reprise après saturation

Construire un service cible vide, de même contexte et époque, sur deux journaux et deux pools
plus grands, puis restaurer WLIV. Le service source peut rester en place jusqu'au succès de cette
opération. L'attente bloque encore `current()` sur la cible ; appeler ensuite `update(pending(),
now, age_us)` pour tenter son admission. La restauration seule ne simule aucun acquittement.
Les deux réserves doivent être dimensionnées pour les transactions futures, pas seulement
pour la première restauration. Aucun agrandissement automatique ni allocation runtime ajoutés.

## Vérification

Trois tests S87, release et suite debug : sauvegarde/restauration identique octet par octet des
trois types de commande bloquée, reprise sur pools élargis ; restauration d'une onde publiée et
comparaison hauteur/vitesse/normale à bits identiques ; toutes les troncatures d'une sauvegarde
à un événement, mutations de version/réserves/contexte/époque, âge physiquement refusé et
temporaire insuffisant, sans modification de la sauvegarde active ; attente d'époque invalide
refusée sans effacer le blocage existant. Pas de nouvelle réception physique.

I-03, I-06 et I-08 inchangés. L201 ; aucun nouvel angle numéroté.

## Ce qui reste ouvert

WLIV provient d'un stockage hôte de confiance. Ce codec ne fournit ni authentification, ni
checksum, ni protection contre un ancien instantané valide rejoué ; les champs de contexte
ne prouvent pas la géométrie réelle. L'hôte assure intégrité et choix de la sauvegarde.
La version 1 ne certifie pas une compatibilité future entre noyaux numériques modifiés.

S86-1 réalisée dans la bibliothèque. Pas d'écriture disque atomique, d'acquittement réseau,
de file d'attente persistante externe ni de récupération automatique après crash construits.
S72-2 reste partielle : sauvegarder les faits ne justifie pas leur purge.
**S87-1, prochaine session S88 :** raccorder le service dynamique à un scénario hôte de bout
en bout, avec requêtes B+W monde, admissions, sauvegarde, redémarrage et reprise ; mesurer
son coût complet avant de poursuivre l'extension des modèles.
