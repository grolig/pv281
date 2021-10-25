# Opakování z přednášky

## Obsah
1. Lifetimes
2. Plánování procesů
3. Paralelismus v Rustu

## Vzorová aplikace
Vytvořte knihovny pro konkurentní slovník. Slovník je přístupný všem pro čtení, ale v případě zápisu je přístupný pouze jednomu zapisujícímu.

## Bodovaná aplikace
Dle assignment.md.

## Bonusová úloha: Problém kuřáků cigaret
Předpokládejme, že cigareta vyžaduje tři ingredience, aby mohla být vykouřena:
tabák
papír
zápalky

Dále předpokládejme, že kolem stolu jsou tři kuřáci, z nichž každý má nekonečné zásoby jedné ze tří ingrediencí – jeden kuřák má nekonečné zásoby tabáku, další má nekonečné zásoby papíru a třetí má nekonečné zásoby zápalek.

Předpokládejme také, že je zde rozhodčí nekuřák. Rozhodčí umožňuje kuřákovi vyrobit si cigaretu tak, že si vybere naslepo (nedeterministicky) dva kuřáky, vezme jednu položku z jejich zdrojů a umístí tyto položky na stůl. Následně upozorní třetího kuřáka, že provedl tuto akci. Třetí kuřák vezme věci ze stolu a použije je (spolu se svým vlastním zdrojem) pro výrobu cigarety, kterou bude následně chvíli kouřit. Mezitím rozhodčí rozhodne, že stůl je opět prázdný, znovu náhodně vybere dva kuřáky a umístí jejich věci na stůl. Tento proces pokračuje donekonečna.

Kuřáci si neshromažďují věci ze stolu; kuřák si začne opět balit další cigaretu, pouze pokud již dokouřil poslední. Pokud by rozhodčí položil tabák a papír na stůl, zatímco muž se zápalkami kouří, tabák a papír zůstanou nedotčeny na stole, dokud tento kuřák nedokouří svou cigaretu a nesebere je.
