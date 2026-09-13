# S218 — Relevés bruts de borne locale

Passage 1 : compilation/tests simultanés, temps non nominaux. Passages 2 et 3 : base seule, binaire final, exécutions isolées.

## Passage 1

```text
S218 CPU release un fil; source gaussienne preparee, borne locale ADR135; aucune technique GPU/LOD/visibilite/mutualisation; partition inclut reste et reserve; preparation separee
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.526 image_time_admitted=true
fixture=base step=2 rectangles=3072 bound=0.115437612 center_max=0.065433979 reserve=0.000266517 bound_over_reference=1.641532 gain=0.997691 partition_ms=1593.873
fixture=base step=1 rectangles=12288 bound=0.115437612 center_max=0.065723725 reserve=0.000266517 bound_over_reference=1.641532 gain=0.997691 partition_ms=6852.494
fixture=base step=0.5 rectangles=49152 bound=0.112293623 center_max=0.069962092 reserve=0.000266517 bound_over_reference=1.596824 gain=1.025625 partition_ms=37135.399
fixture=lent age=9.806095 reference_s217=0.014352296 global=0.032634243 preparation_ms=7.699 image_time_admitted=true
fixture=lent step=2 rectangles=3072 bound=0.032723587 center_max=0.013622921 reserve=0.000089339 bound_over_reference=2.280025 gain=0.997270 partition_ms=2571.289
fixture=lent step=1 rectangles=12288 bound=0.032723587 center_max=0.014292578 reserve=0.000089339 bound_over_reference=2.280025 gain=0.997270 partition_ms=7788.611
fixture=lent step=0.5 rectangles=49152 bound=0.026652928 center_max=0.014312227 reserve=0.000089339 bound_over_reference=1.857050 gain=1.224415 partition_ms=30048.183
fixture=long age=17.806095 reference_s217=0.066987507 global=0.134346545 preparation_ms=7.767 image_time_admitted=true
fixture=long step=2 rectangles=3072 bound=0.134652480 center_max=0.057233181 reserve=0.000305917 bound_over_reference=2.010113 gain=0.997728 partition_ms=1892.454
fixture=long step=1 rectangles=12288 bound=0.134652480 center_max=0.063497588 reserve=0.000305917 bound_over_reference=2.010113 gain=0.997728 partition_ms=7585.368
fixture=long step=0.5 rectangles=49152 bound=0.116187394 center_max=0.066762142 reserve=0.000305917 bound_over_reference=1.734464 gain=1.156292 partition_ms=26367.812
fixture=base_tard age=18.836567 reference_s217=0.044552851 global=0.116219424 preparation_ms=6.452 image_time_admitted=false
fixture=base_tard step=2 rectangles=3072 bound=0.116488002 center_max=0.044200480 reserve=0.000268573 bound_over_reference=2.614603 gain=0.997694 partition_ms=1541.909
fixture=base_tard step=1 rectangles=12288 bound=0.116488002 center_max=0.042674053 reserve=0.000268573 bound_over_reference=2.614603 gain=0.997694 partition_ms=6336.992
fixture=base_tard step=0.5 rectangles=49152 bound=0.086482756 center_max=0.044006381 reserve=0.000268573 bound_over_reference=1.941127 gain=1.343845 partition_ms=25406.015

```

## Passage 2

```text
S218 CPU release un fil; source gaussienne preparee, borne locale ADR135; aucune technique GPU/LOD/visibilite/mutualisation; partition inclut reste et reserve; preparation separee
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.957 image_time_admitted=true
fixture=base step=2 rectangles=3072 bound=0.115437612 center_max=0.065433979 reserve=0.000266517 bound_over_reference=1.641532 gain=0.997691 partition_ms=1661.728
fixture=base step=1 rectangles=12288 bound=0.115437612 center_max=0.065723725 reserve=0.000266517 bound_over_reference=1.641532 gain=0.997691 partition_ms=6729.315
fixture=base step=0.5 rectangles=49152 bound=0.112293623 center_max=0.069962092 reserve=0.000266517 bound_over_reference=1.596824 gain=1.025625 partition_ms=28208.797

```

## Passage 3

```text
S218 CPU release un fil; source gaussienne preparee, borne locale ADR135; aucune technique GPU/LOD/visibilite/mutualisation; partition inclut reste et reserve; preparation separee
fixture=base age=9.806095 reference_s217=0.070323102 global=0.115171090 preparation_ms=6.279 image_time_admitted=true
fixture=base step=2 rectangles=3072 bound=0.115437612 center_max=0.065433979 reserve=0.000266517 bound_over_reference=1.641532 gain=0.997691 partition_ms=1616.573
fixture=base step=1 rectangles=12288 bound=0.115437612 center_max=0.065723725 reserve=0.000266517 bound_over_reference=1.641532 gain=0.997691 partition_ms=6542.758
fixture=base step=0.5 rectangles=49152 bound=0.112293623 center_max=0.069962092 reserve=0.000266517 bound_over_reference=1.596824 gain=1.025625 partition_ms=27545.679

```

