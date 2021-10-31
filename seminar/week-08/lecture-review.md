# Opakování z přednášky

## Obsah
1. Tokio
2. Serde
3. Network Programming

## Vzorová aplikace
Vytvořte aplikaci miniredis. Ta představuje síťový slovník s perzistencí do souboru.

## Bodovaná aplikace
Dle assignment.md.

## Bonusová úloha: Loterie
Vytvořte síťovou aplikaci loterie. Serverová aplikace bude přijímat sázky do loterie (tah, tiket s čísly). Tah má určitou finanční částku, kterou je možné vyhrát. Pro každý tah vygeneruje náhodná čísla. Generování náhodných čísel simulujte tak, že následující číslo tahu se vygeneruje po 3000ms. V rámci klientké knihovny je možné vytvořit ticket. Server poskytne unikátní číslo ticketu po vytvoření sázky. Vúhru na ticketu je možné ověřit samostatnou funkcí, která provolá server, ten poskytne výherní čísla, a počet shod na ticketu a příslušnou výhru. 
