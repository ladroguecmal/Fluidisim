# ADR-145 — I-06 pour l'hôte graphique : tenue par notre code, comptée pour la pile

- Statut : **actée**, S240, 2026-09-15 ; autonomie technique S71.
- Précise la portée de [I-06](../01_INVARIANTS.md) pour la couche `viewer/`, comme
  [ADR-139](ADR-139-volume-et-plan-oriente-des-contenants.md) l'a fait pour I-08 et V.
- Mesures : [ALLOCATIONS-HOTE-S240](../validation/ALLOCATIONS-HOTE-S240.md).

## Constat

I-06 dit « aucune allocation à l'exécution ». Le cœur la reçoit par un allocateur compteur qui refuse
après `seal`. L'hôte GPU, lui, n'avait **jamais** été mesuré : CADENCE-HOTE-S225 §Suite l'écrivait
— « les allocations de la pile graphique (I-06, toujours non reçues) » — et la feuille de route le
gardait dans les travaux nécessaires de J1 (ADR-131 D6).

Mesuré en S240, scène S235, 960×540, 590 images, trois exécutions :

- **notre code allouait une fois par image** — le contour de l'emprise de la visibilité, 12 032
  octets, 39 % des octets de l'image ;
- **la pile verrouillée alloue 133 fois et 18 509 octets par image**, et ce code n'est pas le nôtre ;
- **ces nombres sont constants** : médiane, p95 et maximum égaux, à l'octet près, sur toutes les
  images et toutes les exécutions ; la boucle d'événements de winit n'alloue rien entre deux images ;
- la gigue CPU du même banc — 4,02 ms médian pour 16,3 ms maximum — **ne s'explique donc pas par
  l'allocation**.

## Décision

1. **Pour `viewer/`, I-06 se lit sur le code du projet** : la boucle d'image n'alloue rien en régime.
   C'est vérifiable, c'est vérifié, et un manquement est un défaut à corriger — celui de S240 l'a été.
2. **Les allocations des dépendances verrouillées en S210/S211 sont comptées et publiées**, non
   interdites. Elles entrent dans l'en-tête de mesure d'ADR-131 D3 au même titre que les techniques
   présentes et absentes.
3. **L'instrument reste dans le binaire** : un allocateur compteur global, sans dépendance, relevé
   aux bornes de phase déjà chiffrées par S225. Ce qui n'est pas compté n'est pas tenu.
4. Cette lecture **ne s'étend à aucune autre couche**. Pour le cœur, I-06 garde sa forme stricte, et
   son allocateur compteur refuse toujours après `seal`.

## Conséquences

- Le budget de 2 ms (ADR-125) peut être discuté sans l'inconnue « et les allocations ? » : elles
  valent 133 par image, constantes, sur cette machine et cette version de wgpu.
- **A265 perd une hypothèse** sans être close : la gigue vient d'ailleurs.
- Une version de wgpu, un backend ou un format nouveaux **redemandent la mesure** : 133 est une
  référence datée, pas une constante.
- Ce que cette décision ne fait pas : nommer le site d'appel d'une allocation de la pile, ni suivre
  la mémoire réellement engagée par le système sous les demandes.

## Réversibilité

Retirer le `#[global_allocator]` rend exactement le chemin de S235 ; les nombres cessent d'exister,
et l'invariant redevient non vérifié pour cette couche.
