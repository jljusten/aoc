/* Advent of Code 2017, Day 1 */

/* Jordan Justen : this file is public domain */

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sum = 0;
    let mut last_ch = input.chars().rev().take(1).nth(0).unwrap();
    for ch in input.chars() {
        if ch == last_ch {
            sum += ch.to_digit(10).unwrap();
        }
        last_ch = ch;
    }
    println!("part1: {}", sum);
}

fn part2() {
    let v: Vec<char> = input.chars().collect();
    let len = v.len();
    let offset = len / 2;
    let mut sum = 0;
    for i in 0..len {
        let ch = v[i];
        let cmp_ch = v[(i + offset) % len];
        if ch == cmp_ch {
            sum += ch.to_digit(10).unwrap();
        }
    }
    println!("part2: {}", sum);
}

#[allow(non_upper_case_globals)]
static input: &str =
    "683763347952248558274598352939674972954641755898815882568823446994735\
     954139126882786472358625661232339839216625787929174539127953527464265\
     126499656159195885121255671868374113711798752876214887597614296291748\
     869722983491977224234582993231415294131913276224852494958641681813271\
     976614544649263262482749994483737418399631556468288427527612931423564\
     229643553495219874832114963612896663757797283459522316494537116845391\
     648931518118496533318459989985979911468813617172345179117598937923488\
     158187552624563786271167794954355961396172465716785311833359562441638\
     714456742447655864463625291598541375359621171848751922738722228998873\
     572923129782861826362329212525747381183475211876378296238318724373819\
     792239556756342578891378236849241273384332485195152117967325993149216\
     113997365712772225463323694611362774174197948655241239897224923565368\
     323139375974377178737875938494688367336425293785471511463975329972374\
     393876637693347229791729548351544863829837166982126943573981533929262\
     552729613846261318296781712195692886855971411323553227882541639238883\
     781555739487531854231589978777186876424464574466434225365412389797617\
     254964262923593821685356412161242117418965525621289418241722419138735\
     378289761727382769839152322414515894219111215672288998539346679547862\
     562236146215546182944671912551533952565247861597584296437565864576391\
     771838911622141635496885954168933831949958245342478414142475262682127\
     619549137194521148767647457999827925947537596263343196311919178943681\
     167388935487976611118996641383983548189311354869849447199923931486817\
     241166167414289376879851526582966798454747664777415536327129686791753\
     564529874597611264372167581711823952193932891991489968137628499914846\
     784297935786293312157969967514843757848955616821566585798875187468623\
     717513726924727652173747913246567452915747844952994773629646763511481\
     836768971223668386563427459449452752636177293598314665656949832172525\
     942378281876128575233442654182278832193831388938733847756595486376628\
     675726871982636885978651181739216151781654421339873623827214448449527\
     15592955744739873677838847693982379696776";
