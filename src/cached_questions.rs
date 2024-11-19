//! Cached version of the JSON object representing all questions (so we don't have to use fs in
//! wasm)

/// A static constant string representing all questions
pub const CACHED_QUESTIONS: &str = r#"
{
    "questions": [
        {
            "name": "What is the capital of Afghanistan?",
            "answers": [
                "Tirana",
                "Kabul",
                "Dushanbe",
                "Tashkent"
            ],
            "correct": "Kabul"
        },
        {
            "name": "What is the capital of Australia?",
            "answers": [
                "Canberra",
                "Sydney",
                "Melbourne",
                "Ottawa"
            ],
            "correct": "Canberra"
        },
        {
            "name": "What is the capital of Belgium?",
            "answers": [
                "Amsterdam",
                "Luxemburg",
                "Brussels",
                "Stockholm"
            ],
            "correct": "Brussels"
        },
        {
            "name": "What is the capital of Greece?",
            "answers": [
                "Ankara",
                "Athens",
                "Sofia",
                "Thessaloniki"
            ],
            "correct": "Athens"
        },
        {
            "name": "What is the capital of Italy?",
            "answers": [
                "Venice",
                "Rome",
                "Naples",
                "Milan"
            ],
            "correct": "Rome"
        },
        {
            "name": "What is the capital of Israel?",
            "answers": [
                "Tel Aviv",
                "Kabul",
                "Jerusalem",
                "Islamabad"
            ],
            "correct": "Jerusalem"
        },
        {
            "name": "What is the capital of Germany?",
            "answers": [
                "Frankfurt",
                "Berlin",
                "Munich",
                "Hamburg"
            ],
            "correct": "Berlin"
        },
        {
            "name": "What is the capital of Norway?",
            "answers": [
                "Stockholm",
                "Helsinki",
                "Oslo",
                "Copenhagen"
            ],
            "correct": "Oslo"
        },
        {
            "name": "What is the capital and largest city of Hawaii, the 51th US state?",
            "answers": [
                "Little Rock",
                "Dover",
                "Frankfort",
                "Honolulu"
            ],
            "correct": "Honolulu"
        },
        {
            "name": "When the streams Biya and Katun join in Altai Krai, they form this mighty river. It is located in West Siberia, Russia and has many names- the Siberian Tatars call it Omar or Umar, the Samoyedes- Kolta or Kuay and to the Ostiaks it is known as the As, Yag, Kolta and Yema. It joins the Irtysh river, forming the longest river flow in Russia. What is its name?",
            "answers": [
                "Ural",
                "Volga",
                "Ob",
                "Lena"
            ],
            "correct": "Ob"
        },
        {
            "name": "Although the Amazon river is generally regarded as the second-longest in the world, it is the river with greatest total flow, carrying more than the Mississippi, Nile, and Yangtze rivers combined.  It ends in the Atlantic Ocean, but it is believed to begin its long journey from this mountain peak.",
            "answers": [
                "Nevado Mismi",
                "Misti",
                "Cotopaxi",
                "Mount Chimborazo"
            ],
            "correct": "Nevado Mismi"
        },
        {
            "name": "This is the longest river in Asia and its Chinese name, Chang Jiang, is literally translated to Long River. The critically endangered Chinese River Dolphin and Chinese paddlefish live only in this river. It takes its source in the Qinghai Province, flows for 6,380km (3964miles) and finally empties into the East China Sea. What is the name of this river?",
            "answers": [
                "Irtysh",
                "Huai River",
                "Yangtze",
                "Mekong"
            ],
            "correct": "Yangtze"
        },
        {
            "name": "Huang He is the second-longest river in China. Its source is in the Kunlun Mountains at 4,500m (14,764 feet) above sea level. Due to the silts that the river carries, the color of its waters becomes so unnatural that it gave the name of the river . What is the color of the waters of Huang He?",
            "answers": [
                "Brown",
                "Orange",
                "Yellow",
                "Red"
            ],
            "correct": "Yellow"
        },
        {
            "name": "It is the second-longest river in the United States. The longest one, Missouri, joins it to form the longest river flow in North America. This river flows through ten states- Minnesota, Wisconsin, Iowa, Illinois, Missouri, Kentucky, Arkansas, Tennessee, Mississippi and Louisiana and mouths into the Gulf of Mexico. I am not going to ask you about its name, but about its source. What is the origin of the Mississippi river?",
            "answers": [
                "Lake Superior",
                "Lake Pepin",
                "Lake Itasca",
                "Cass Lake"
            ],
            "correct": "Lake Itasca"
        },
        {
            "name": "This is the twelfth-longest river in the world. It runs through China, Myanmar, Thailand, Laos, Cambodia and Vietnam. There are various theories about the source and the exact length of the river because there are several effluents of it that are too difficult to explore. Due to the many rapids and waterfalls, as well as the extreme changes in the flow of the river, sailing is extremely difficult. What is the name of this river?",
            "answers": [
                "Mekong",
                "Saskatchewan",
                "Angara",
                "Brahmaputra"
            ],
            "correct": "Mekong"
        },
        {
            "name": "The longest river in Europe is Volga. But do you know which is the second longest one? It flows through several major European cities, such as Ulm, Vienna, Bratislava, Budapest and Belgrade. The river empties in the Black Sea on the terrirories of Romania and Ukraine.",
            "answers": [
                "Don",
                "Dniepr",
                "Danube",
                "Emba"
            ],
            "correct": "Danube"
        },
        {
            "name": "The Ganges river is considered to be a holy in India. There is a Hindu legend that the river was created from the sweat of the feet of Vishnu, collected by Brahma. According to Hindu beliefs, if you bathe in the waters of Ganges, it will wash away your sins. However this is not very hygienic, as the river is badly polluted by cremated corpses, carcasses, waste from factories and more.  Where does this river empty?",
            "answers": [
                "The Bay of Bengal",
                "Lop Nur",
                "The Quarry Bay",
                "Kara Sea"
            ],
            "correct": "The Bay of Bengal"
        },
        {
            "name": "Victoria Falls is one of the most spectacular waterfalls in the world. The falls are named after Queen Victoria by David Livingstone, the explorer who visited them in 1855. The falls are 128 m (420 ft) high and are situated on this river.",
            "answers": [
                "Zambezi",
                "Congo",
                "Gambia",
                "Orange"
            ],
            "correct": "Zambezi"
        },
        {
            "name": "The Nile is generally considered the longest river in the world.  The source of this mighty river remained unknown for centuries. Finally Lake Victoria was decided to be its source, although there are other theories. Lake Victoria is located on the territories of these three countries.",
            "answers": [
                "Sudan, Ethiopia and Kenya",
                "Zambia, Angola and Sudan",
                "Uganda, Kenya and Tanzania",
                "Egypt, Morocco and Zimbabwe"
            ],
            "correct": "Uganda, Kenya and Tanzania"
        },
        {
            "name": "Name the line, which is the same distance from the North Pole and South Pole and runs horizontally around the world.",
            "answers": [
                "Tropic of Capricorn",
                "Equator",
                "Tropic of Cancer",
                "Prime Meridian"
            ],
            "correct": "Equator"
        },
        {
            "name": "What term refers to the horizontal line, 23.5 degrees S, which passes through South America, Africa, and Australia?",
            "answers": [
                "Tropic of Cancer",
                "Prime Meridian",
                "Tropic of Capricorn",
                "Equator"
            ],
            "correct": "Tropic of Capricorn"
        },
        {
            "name": "Which continents are entirely in the Southern Hemisphere?",
            "answers": [
                "South America, Africa, and Australia",
                "Australia and Antarctica",
                "South America, Australia and Antarctica",
                "South America and Australia"
            ],
            "correct": "Australia and Antarctica"
        },
        {
            "name": "Which continents (landmasses, not peripheral islands) are entirely in the Northern Hemisphere?",
            "answers": [
                "North America and Europe",
                "Europe, Africa and Asia",
                "North America, Africa, and Asia",
                "North America, Europe, and Asia"
            ],
            "correct": "North America, Europe, and Asia"
        },
        {
            "name": "Which continents are entirely in the Western Hemisphere?",
            "answers": [
                "North America, Europe, and Asia",
                "Europe, Asia, Africa, Australia",
                "Australia and Antarctica",
                "North America and South America"
            ],
            "correct": "North America and South America"
        },
        {
            "name": "Which feature on a map helps determine direction?",
            "answers": [
                "a key",
                "a legend",
                "a compass rose",
                "a scale"
            ],
            "correct": "a compass rose"
        },
        {
            "name": "Which of these is not a type of map projection?",
            "answers": [
                "Robinson",
                "Crusoe",
                "Mercator",
                "Interrupted"
            ],
            "correct": "Crusoe"
        },
        {
            "name": "Which of these continents is largest (by territory)?",
            "answers": [
                "South America",
                "Antarctica",
                "North America",
                "Europe"
            ],
            "correct": "North America"
        },
        {
            "name": "Which of these countries is smallest (by territory)?",
            "answers": [
                "Seychelles",
                "Liechtenstein",
                "San Marino",
                "Marshall Islands"
            ],
            "correct": "San Marino"
        },
        {
            "name": "Which of these US cities is largest (by population)?",
            "answers": [
                "Houston",
                "Phoenix",
                "Philadelphia",
                "San Antonio"
            ],
            "correct": "Houston"
        },
        {
            "name": "Which of these mountains is highest?",
            "answers": [
                "Puncak Jaya",
                "Mount McKinley",
                "Mount Elbrus",
                "Mount Kilimanjaro"
            ],
            "correct": "Mount McKinley"
        },
        {
            "name": "Which of these lakes is deepest?",
            "answers": [
                "Caspian Sea",
                "Malawi or Nyasa",
                "Tanganyika",
                "Issyk-Kul"
            ],
            "correct": "Tanganyika"
        },
        {
            "name": "Which of these rivers is longest?",
            "answers": [
                "Lena",
                "Huang",
                "Amur",
                "Ob-Irtysh"
            ],
            "correct": "Huang"
        },
        {
            "name": "Which of these seas is largest?",
            "answers": [
                "Bering",
                "Arabian Sea",
                "Mediterranean",
                "Gulf of Mexico"
            ],
            "correct": "Mediterranean"
        },
        {
            "name": "Which of these deserts is largest?",
            "answers": [
                "Chihuahuan",
                "Syrian",
                "Thar",
                "Great Sandy"
            ],
            "correct": "Syrian"
        },
        {
            "name": "Which of these inhabited places is wettest?",
            "answers": [
                "Moulein",
                "Sylhet",
                "Baguio",
                "Lae"
            ],
            "correct": "Moulein"
        },
        {
            "name": "Which of these oceans has the greatest depth?",
            "answers": [
                "Pacific Ocean",
                "Indian Ocean",
                "Atlantic Ocean",
                "Arctic Ocean"
            ],
            "correct": "Pacific Ocean"
        },
        {
            "name": "The rivers - the Tajo, the Ebro, the Duero, and the Guadiana, are located in this country.",
            "answers": [
                "Columbia",
                "Portugal",
                "Spain",
                "Brazil"
            ],
            "correct": "Spain"
        },
        {
            "name": "The volcanoes - Akan, Aso, Mount Fuji and Rausu are located in this country.",
            "answers": [
                "Japan",
                "Malaysia",
                "North Korea",
                "Indonesia"
            ],
            "correct": "Japan"
        },
        {
            "name": "Onega, Khanka and Chudskoye are three of the many lakes in this country.",
            "answers": [
                "Kazakhstan",
                "Mongolia",
                "Ukraine",
                "Russia"
            ],
            "correct": "Russia"
        },
        {
            "name": "The valleys of the Yellow River and the Pearl River are two of the seven main national valleys in this country.",
            "answers": [
                "Burma",
                "China",
                "Thailand",
                "India"
            ],
            "correct": "China"
        },
        {
            "name": "Pico da Bandeira, Pico do Cruzeiro and Pedra da Mina are three of the numerous mountains located in this South American country.",
            "answers": [
                "Argentina",
                "Chili",
                "Brazil",
                "Peru"
            ],
            "correct": "Brazil"
        },
        {
            "name": "Chauvet Cave and Meyrieres Cave are two caves located in this European state.",
            "answers": [
                "France",
                "Netherlands",
                "Spain",
                "Belgium"
            ],
            "correct": "France"
        },
        {
            "name": "Kainji Lake and Lake Chad are considered lakes of this country.",
            "answers": [
                "Niger",
                "Nigeria",
                "Cameroon",
                "Chad"
            ],
            "correct": "Nigeria"
        },
        {
            "name": "Mitchell, Jardine, Staaten, Flinders, Leichhardt, and Nicholson are just few of the rivers in this country.",
            "answers": [
                "Indonesia",
                "Australia",
                "Papua New Guinea",
                "New Zealand"
            ],
            "correct": "Australia"
        },
        {
            "name": "Kaskaspakte, Akka and Sielmmacohkka are three mountains in this country.",
            "answers": [
                "Norway",
                "Finland",
                "Denmark",
                "Sweden"
            ],
            "correct": "Sweden"
        },
        {
            "name": "Dasht-e Kavir and Kavir-e Lut are deserts located in this Asian country.",
            "answers": [
                "India",
                "Iran",
                "Pakistan",
                "Iraq"
            ],
            "correct": "Iran"
        },
        {
            "name": "Which two countries border the Dead Sea?",
            "answers": [
                "Jordan and Israel",
                "Lebanon and Jordan",
                "Syria and Jordan",
                "Lebanon and Israel"
            ],
            "correct": "Jordan and Israel"
        },
        {
            "name": "Is it true that Yasseir Arafat became chairman of the Palestinian Liberation Organization in 2004?",
            "answers": [
                "Yes",
                "No",
                null,
                null
            ],
            "correct": "No"
        },
        {
            "name": "What are the three Benelux countries?",
            "answers": [
                "Belgium, Netherlands and Luxembourg",
                "Finland, Sweden and Denmark",
                "The U.S.A., Canada and Mexico",
                "Honduras, Nicaragua and Belize"
            ],
            "correct": "Belgium, Netherlands and Luxembourg"
        },
        {
            "name": "Did the 13 colonies declare their independence in 1776?",
            "answers": [
                "No",
                "Yes",
                null,
                null
            ],
            "correct": "Yes"
        },
        {
            "name": "Europe is the smallest continent.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "What are the Tropic of Cancer and the Tropic of Capricorn?",
            "answers": [
                "Lines of latitude",
                "Lines of longitude",
                "Parallel to the Greenwhich Meridian",
                "None of these"
            ],
            "correct": "Lines of latitude"
        },
        {
            "name": "Which statement about the emu and the kangaroo is not true?",
            "answers": [
                "Both are indigenous only to Australia.",
                "Both are on the national emblem of Australia.",
                "Both are incapable of flying.",
                "Both are incapable of  walking backwards."
            ],
            "correct": "Both are indigenous only to Australia."
        },
        {
            "name": "What was the former name of the city of Istanbul ?",
            "answers": [
                "Constantinople",
                "Persia",
                "Zaire",
                "Siam"
            ],
            "correct": "Constantinople"
        },
        {
            "name": "Where would you find the shortest river in the world?",
            "answers": [
                "Wales",
                "Ecuador",
                "Tasmania",
                "Montana"
            ],
            "correct": "Montana"
        },
        {
            "name": "What major geographical feature is located at 23.5 degrees North latitude?",
            "answers": [
                "The Equator",
                "The Arctic Circle",
                "The Tropic of Capricorn",
                "The Tropic of Cancer"
            ],
            "correct": "The Tropic of Cancer"
        },
        {
            "name": "Which would be found at 39 degrees 43 minutes North latitude?",
            "answers": [
                "The Tropic of Cancer",
                "The Tropic of Capricorn",
                "The majority of the Mason-Dixon Line.",
                "The border between Canada and  Washington State."
            ],
            "correct": "The majority of the Mason-Dixon Line."
        },
        {
            "name": "Which country held the record for the highest birth rate in the 2005 world statistics?",
            "answers": [
                "Niger",
                "Uganda",
                "Mali",
                "Vatican City"
            ],
            "correct": "Niger"
        },
        {
            "name": "How many US states border the Gulf of Mexico?",
            "answers": [
                "4",
                "3",
                "5",
                "10"
            ],
            "correct": "5"
        },
        {
            "name": "Measuring at the Equator, what is the closest approximation of the Earths circumference?",
            "answers": [
                "24,900,000 miles",
                "24,900 miles",
                "249,000 miles",
                "2,490,000 miles"
            ],
            "correct": "24,900 miles"
        },
        {
            "name": "The people of which country watched the most TV per week in 2005?",
            "answers": [
                "Thailand",
                "The Philippines",
                "The USA",
                "Egypt"
            ],
            "correct": "Thailand"
        },
        {
            "name": "How many US states border the Pacific Ocean?",
            "answers": [
                "11",
                "4",
                "5",
                "3"
            ],
            "correct": "5"
        },
        {
            "name": "Which one of these ski resorts is not located in the Alps?",
            "answers": [
                "Kitzbuhel",
                "Zermatt",
                "Mont-Tremblant",
                "Chamonix-Mont-Blanc"
            ],
            "correct": "Mont-Tremblant"
        },
        {
            "name": "The tallest mountain peak in the US is Mount McKinley. This is the second tallest.",
            "answers": [
                "Mount Saint Elias",
                "Glacier Peak",
                "Mount Washington",
                "Gannett Peak"
            ],
            "correct": "Mount Saint Elias"
        },
        {
            "name": "Which one of these mountains is not a volcano?",
            "answers": [
                "Mauna Kea",
                "Mount St. Helens",
                "Mount Vesuvius",
                "Mount Aspiring"
            ],
            "correct": "Mount Aspiring"
        },
        {
            "name": "The highest mountain peak on our planet is Mount Everest. It was first climbed by Edmund Hillary and Tenzing Norgay on May 29, 1953. What is the home country of Sir Edmund Hillary?",
            "answers": [
                "United Kingdom",
                "The United States",
                "France",
                "New Zealand"
            ],
            "correct": "New Zealand"
        },
        {
            "name": "Which saltwater lake, bordered on the west by Azerbaijan and Russia, is the largest inland body of water in the world?",
            "answers": [
                "Tanganyika",
                "Victoria",
                "Baikal",
                "Caspian Sea"
            ],
            "correct": "Caspian Sea"
        },
        {
            "name": "This country, situated in North Europe, has almost 200 000 lakes - more than any other country in the world.",
            "answers": [
                "Finland",
                "Sweden",
                "The Netherlands",
                "Georgia"
            ],
            "correct": "Finland"
        },
        {
            "name": "Which body of water, situated in Southern Siberia, is the deepest and oldest freshwater lake on Earth?",
            "answers": [
                "Onega",
                "Ladoga",
                "Huron",
                "Baikal"
            ],
            "correct": "Baikal"
        },
        {
            "name": "This water body, which is South Americas largest freshwater lake, is the highest commercially navigable lake.",
            "answers": [
                "Nyasa",
                "Nicaragua",
                "Titicaca",
                "Michigan"
            ],
            "correct": "Titicaca"
        },
        {
            "name": "This country has more than 60% of worlds lakes due to its special drainage system.",
            "answers": [
                "Australia",
                "Russia",
                "Canada",
                "The USA"
            ],
            "correct": "Canada"
        },
        {
            "name": "This freshwater-lake island, with a surface area of 2,766 km², is the biggest on Earth.",
            "answers": [
                "Islandlake",
                "Ainslie",
                "Manitoulin Island",
                "Isle of Wight"
            ],
            "correct": "Manitoulin Island"
        },
        {
            "name": "What Canadian lake is the largest in the world, located on an island?",
            "answers": [
                "Nettilling Lake",
                "Aral",
                "Winnipeg",
                "Michigan"
            ],
            "correct": "Nettilling Lake"
        },
        {
            "name": "This is the lowest lake on Earth, situated at about 400 meters below sea level on the border between Israel and Jordan.",
            "answers": [
                "Issyk-Kul",
                "Balkhash",
                "Urmia",
                "Dead Sea"
            ],
            "correct": "Dead Sea"
        },
        {
            "name": "Which tiny lake located in Tibet is the highest lake on Earth?",
            "answers": [
                "Lhagba Pool",
                "Rush Lake",
                "Laguna Lobato",
                "Poquentica Lake"
            ],
            "correct": "Lhagba Pool"
        },
        {
            "name": "With a surface area of about 82,000 km², this lake between Ontario and Minnesota is the largest single freshwater lake in the world.",
            "answers": [
                "Turkana",
                "Michigan",
                "Lake Superior",
                "Onega"
            ],
            "correct": "Lake Superior"
        },
        {
            "name": "The word volcano comes from the name of the Roman god of fire, Vulcan. Who is the analogue of Vulcan in Greek mythology?",
            "answers": [
                "Hephaestus",
                "Helios",
                "Hermes",
                "Zeus"
            ],
            "correct": "Hephaestus"
        },
        {
            "name": "If magma contains more than 65% silica, the lava is called this.",
            "answers": [
                "felsic or acidic",
                "thick",
                "mafic or basic",
                "dense"
            ],
            "correct": "felsic or acidic"
        },
        {
            "name": "Phreatic eruptions occur when the temperature of magma is relatively low and it solidifies very quickly, sometimes blocking the vent of the volcano.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Yellowstone Caldera is considered a supervolcano because within the past two million years it has erupted extremely violently three times . The most recent eruption occurred this long ago, and spread volcanic ash over most of the North American continent.",
            "answers": [
                "1,510,000 years",
                "40,000 years",
                "640,000 years",
                "17,000 years"
            ],
            "correct": "640,000 years"
        },
        {
            "name": "Which of the following states of America is an inland one and has an official motto that does not come from Latin?",
            "answers": [
                "Montana",
                "Virginia",
                "Oklahoma",
                "South Carolina"
            ],
            "correct": "Montana"
        },
        {
            "name": "Which of the following states has an official motto In God we trust, which is also the official motto of the United States of America?",
            "answers": [
                "Georgia",
                "Washington",
                "South Dakota",
                "Florida"
            ],
            "correct": "Florida"
        },
        {
            "name": "Three of these US states have an official motto related to liberty. Spot the odd one.",
            "answers": [
                "Iowa",
                "Oklahoma",
                "New Jersey",
                "New Hampshire"
            ],
            "correct": "Oklahoma"
        },
        {
            "name": "This famous Latin maxim, which is the official motto of South Carolina, means While I breathe, I hope.",
            "answers": [
                "Si quaeris peninsulam amoenam circumspice",
                "Nil sine numine",
                "Salus populi suprema lex esto",
                "Dum spiro spero"
            ],
            "correct": "Dum spiro spero"
        },
        {
            "name": "Three of the following US states have a motto consisting of one single word in English. Which one does not belong to the group?",
            "answers": [
                "Utah",
                "Texas",
                "West Virginia",
                "Rhode Island"
            ],
            "correct": "West Virginia"
        },
        {
            "name": "North to the Future is the official motto of this US state, which is the largest in area.",
            "answers": [
                "Texas",
                "Alaska",
                "Nevada",
                "Colorado"
            ],
            "correct": "Alaska"
        },
        {
            "name": "Which of the following maxims, meaning The people rule, is the official state motto of Arkansas?",
            "answers": [
                "Esto perpetua",
                "Regnat populus",
                "Dirigo",
                "Ditat Deus"
            ],
            "correct": "Regnat populus"
        },
        {
            "name": "This famous exclamation, which has been attributed to ancient scientist Archimedes, is the official state motto of California.",
            "answers": [
                "Excelsior!",
                "Virtute et armis",
                "Ad astra per aspera",
                "Eureka"
            ],
            "correct": "Eureka"
        },
        {
            "name": "The official motto of this state in the Pacific Northwest of the United States comes from Chinook Jargon.",
            "answers": [
                "Wisconsin",
                "Washington",
                "Pennsylvania",
                "New York"
            ],
            "correct": "Washington"
        },
        {
            "name": "Which of these states has an official motto that doesnt include the word rights in it?",
            "answers": [
                "Iowa",
                "Indiana",
                "Wyoming",
                "Alabama"
            ],
            "correct": "Indiana"
        },
        {
            "name": "Which volcano, located on Ross Island, Antarctica, is the southernmost active volcano on Earth?",
            "answers": [
                "Mayon Volcano",
                "Cold Bay Volcano",
                "Mount Erebus",
                "Teide"
            ],
            "correct": "Mount Erebus"
        },
        {
            "name": "This volcano, which is the largest in Europe, is located on the largest island in the Mediterranean Sea.",
            "answers": [
                "Mount Etna",
                "Hekla",
                "El Misti",
                "Santorini"
            ],
            "correct": "Mount Etna"
        },
        {
            "name": "Popocatépetl, a volcano whose name means Smoking Mountain, is 70 km away from the capital of which American country?",
            "answers": [
                "Mexico",
                "Peru",
                "Canada",
                "Argentina"
            ],
            "correct": "Mexico"
        },
        {
            "name": "Mount Vesuvius, that is notorious for the destruction of two Roman cities in the year 79, was linked most closely to which hero and demigod?",
            "answers": [
                "Asclepius",
                "Apollo",
                "Zeus",
                "Hercules"
            ],
            "correct": "Hercules"
        },
        {
            "name": "What is the biggest extinct volcano in Ecuador, situated in the Andes mountain.",
            "answers": [
                "Chimborazo",
                "Ojos Del Salado",
                "Llullaillaco",
                "Cotopaxi"
            ],
            "correct": "Chimborazo"
        },
        {
            "name": "This dormant volcano, which is 66 km northeast of Tehran, is the symbol of Iranian resistance against foreign rule.",
            "answers": [
                "Mount Hood",
                "Krakatoa",
                "Mount Damavand",
                "Krafla"
            ],
            "correct": "Mount Damavand"
        },
        {
            "name": "This active volcano, whose name means long mountain in Hawaiian, is one of the five peaks that form the Island of Hawaii.",
            "answers": [
                "Cotopaxi",
                "Mauna Loa",
                "Mount Kea",
                "Hekla"
            ],
            "correct": "Mauna Loa"
        },
        {
            "name": "What volcano located on the island of Tenerife represents the highest mountain on Spanish territories?",
            "answers": [
                "Teide",
                "Stromboli",
                "Santorini",
                "Duvalo"
            ],
            "correct": "Teide"
        },
        {
            "name": "The name of this volcano, located on the Alaska Peninsula, means new eruption.",
            "answers": [
                "Newatsa",
                "Mount Rainier",
                "Novarupta",
                "Mount Hood"
            ],
            "correct": "Novarupta"
        },
        {
            "name": "Which is the volcanically active region that is situated in a famous national US park and has a territory of 55 kilometers by 72 kilometers?",
            "answers": [
                "Yosemite Valley",
                "Yellowstone Caldera",
                "Mount Shasta",
                "Cold Bay Volcano"
            ],
            "correct": "Yellowstone Caldera"
        },
        {
            "name": "Where is the southernmost point in all 50 states?",
            "answers": [
                "Texas",
                "Hawaii",
                "Florida",
                "California"
            ],
            "correct": "Hawaii"
        },
        {
            "name": "And where is the lowest point in the USA, North America, and western hemisphere? I need the state and national park name.",
            "answers": [
                "Maine, Acadia NP",
                "California, Death Valley NP",
                "Arizona, Grand Canyon NP",
                "Wyoming, Yellowstone NP"
            ],
            "correct": "California, Death Valley NP"
        },
        {
            "name": "The state with the largest area in the lower 48 is?",
            "answers": [
                "California",
                "Texas",
                "Montana",
                "Arizona"
            ],
            "correct": "Texas"
        },
        {
            "name": "England, Norway, Belgium and Denmark all border this sea.",
            "answers": [
                "Baltic sea",
                "Black sea",
                "North sea",
                "Aegean sea"
            ],
            "correct": "North sea"
        },
        {
            "name": "Towards the beginning of the new millennium, three of these four countries had a population of approximately 32 million people. Which is the odd one?",
            "answers": [
                "Algeria",
                "Kenya",
                "Colombia",
                "Morocco"
            ],
            "correct": "Colombia"
        },
        {
            "name": "Canada, the United States and China are three countries whose total area is approximately 12 million sq km.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "The US is one of the leading countries in the number plastic surgery procedures.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "In the beginning of the new millennium, Russias, Iraqs and Saudi Arabias oil reserves were above 100 billion bbl.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "USA is considered to be one of the most popular destinations, where thousands of immigrants come to seek asylum. Just one of these four states is similar in to USA in this respect.",
            "answers": [
                "France",
                "Finland",
                "Japan",
                "Portugal"
            ],
            "correct": "France"
        },
        {
            "name": "Ireland, Germany and Austria are famous for the high consumption of this alcohol.",
            "answers": [
                "Brandy",
                "Beer",
                "Wine",
                "Vodka"
            ],
            "correct": "Beer"
        },
        {
            "name": "Dominica, New Zealand and Finland are three countries with high crime rates.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "All of these countries and commonwealth territories have a high divorce rate except one.",
            "answers": [
                "Puerto Rico",
                "Russia",
                "Sri Lanka",
                "Qatar"
            ],
            "correct": "Sri Lanka"
        },
        {
            "name": "This island is well known in scientific fields for being home of five percent of the worlds plant and animal species, with 80% of them unique to it. The island is also the fourth biggest on Earth, with population of about 17,500,000 people.",
            "answers": [
                "Greenland",
                "Madagascar",
                "New Guinea",
                "Tasmania"
            ],
            "correct": "Madagascar"
        },
        {
            "name": "In Greek mythology, this island is the legendary birthplace of the goddess of beauty, love, and passion, the charming Aphrodite, where according to the legend, she emerged from the sea foam.",
            "answers": [
                "Crete",
                "Cyprus",
                "Limnos",
                "Rhodes"
            ],
            "correct": "Cyprus"
        },
        {
            "name": "Because of its spectacular nature forms, one of this insular countrys islands has been a set for the filming of many movie productions, including the highly successful, Lord of the Rings and The Chronicles of Narnia.",
            "answers": [
                "The Philippines",
                "New Zealand",
                "Indonesia",
                "New Guinea"
            ],
            "correct": "New Zealand"
        },
        {
            "name": "Despite being the largest in the world, this island is sparsely populated, due to its unfavourable climate.",
            "answers": [
                "Greenland",
                "New Guinea",
                "Victoria Island",
                "Madagascar"
            ],
            "correct": "Greenland"
        },
        {
            "name": "Being the most populous in the world, with bigger population than Australia and Antarctica, the density of this islands inhabitants forced the countrys government to start transmigration programs, aimed at resettling people to other, less-crowded parts of the country.",
            "answers": [
                "Java",
                "Sumatra",
                "Borneo",
                "Sulawesi"
            ],
            "correct": "Java"
        },
        {
            "name": "This is the largest island in the Mediterranean Sea, famous for one of the worlds most active volcanoes - Etna.",
            "answers": [
                "Sicily",
                "Crete",
                "Sardinia",
                "Corse"
            ],
            "correct": "Sicily"
        },
        {
            "name": "Al Nakheel Properties, one of the leading real estate developers in Dubai and creator of the worldwide famous Palm Islands, is also the initiator of another grand project of artificial islands, which resembles the shape of what?",
            "answers": [
                "A seashell",
                "The globe",
                "A human face",
                "A dolphin"
            ],
            "correct": "The globe"
        },
        {
            "name": "Which of the following is a traditional Vietnamese clothing, worn primarily by women?",
            "answers": [
                "Qipao",
                "Áo dài",
                "Nhu",
                "Raglan"
            ],
            "correct": "Áo dài"
        },
        {
            "name": "The gowni is a typical national costume in which African country?",
            "answers": [
                "Tanzania",
                "Malawi",
                "Uganda",
                "Indonesia"
            ],
            "correct": "Tanzania"
        },
        {
            "name": "These knee-breeches made of leather are a traditional Bavarian garment.",
            "answers": [
                "Kilt",
                "Lederhosen",
                "Kisen",
                "Wrestlers"
            ],
            "correct": "Lederhosen"
        },
        {
            "name": "Hijab, which is also a term meaning barrier, is worn as a traditional clothing in which countries?",
            "answers": [
                "African",
                "European",
                "Arabic",
                "Latin"
            ],
            "correct": "Arabic"
        },
        {
            "name": "Which of these national costumes is closely related to Sikhism - a religion that comes from 16th-century northern India?",
            "answers": [
                "Khalsa",
                "Sari",
                "Turban",
                "Sayyid"
            ],
            "correct": "Turban"
        },
        {
            "name": "Barong is the embroidered formal garment of which country in South-East Asia?",
            "answers": [
                "Russia",
                "Indonesia",
                "The Philippines",
                "Japan"
            ],
            "correct": "The Philippines"
        },
        {
            "name": "Which of these national costumes does not belong to a European nation?",
            "answers": [
                "Hanfu",
                "Tracht",
                "Kilt",
                "Kroje"
            ],
            "correct": "Hanfu"
        },
        {
            "name": "Which one of the following national costumes is not Chinese?",
            "answers": [
                "Kente cloth",
                "Qipao",
                "Cheongsam",
                "Mao suit"
            ],
            "correct": "Kente cloth"
        },
        {
            "name": "Which of these pieces of clothing is worn on the head?",
            "answers": [
                "Fez",
                "Mitumba",
                "Koto",
                "Kiondo"
            ],
            "correct": "Fez"
        },
        {
            "name": "Three of the enlisted national garments are African, while the fourth one is Norwegian. Which one?",
            "answers": [
                "Sanafil",
                "Bunad",
                "Dashiki",
                "Boubou"
            ],
            "correct": "Bunad"
        },
        {
            "name": "According to the legend, which of these is a part of the Three Fortunate Concealments that protect Britain from any danger coming from the sea?",
            "answers": [
                "The head of Bran the Blessed",
                "Stone of Destiny",
                "The Dragons",
                "The bones of Gwerthefyr the Blessed"
            ],
            "correct": "The head of Bran the Blessed"
        },
        {
            "name": "While driving or walking down Kings Cross Road and Farringdon Road in London few people are aware that unseen beneath them runs which river?",
            "answers": [
                "River Thames",
                "River Avon (Shakespeares Avon)",
                "River Rhodda",
                "River Fleet"
            ],
            "correct": "River Fleet"
        },
        {
            "name": "Who made the statue of King Richard I outside the Houses of Parliament in Old Palace Yard in London?",
            "answers": [
                "John Doubleday",
                "Baron Carlo Marochetti",
                "Sir George Frampton",
                "Joseph Edgar Boehm"
            ],
            "correct": "Baron Carlo Marochetti"
        },
        {
            "name": "The Great Pavement, a hidden, unknown to most tourists and seldom-displayed treasure, is located in which building in London?",
            "answers": [
                "Buckingham Palace",
                "Tate Gallery",
                "St Jamess Palace",
                "Westminster Abbey"
            ],
            "correct": "Westminster Abbey"
        },
        {
            "name": "The Jewel Tower, built in 1366, is physically separated from the Palace of Westminster in London.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The famous department store Harrods that occupies an entire block is located on which road in London?",
            "answers": [
                "Abbey Road",
                "Threadneedle Street",
                "Brompton Road",
                "Fleet Street"
            ],
            "correct": "Brompton Road"
        },
        {
            "name": "The Old Royal Observatory, which is a part of the National Maritime Museum in London, was built in what year?",
            "answers": [
                "1675",
                "1765",
                "1567",
                "1657"
            ],
            "correct": "1675"
        },
        {
            "name": "This famous writer, whose house was at 17 Gough Square in London, said: When a man is tired of London, he is tired of life, for there is in London all life can afford.",
            "answers": [
                "Charles Dickens",
                "Dr Samuel Johnson",
                "Thomas Chestre",
                "Tomas John Dibdin"
            ],
            "correct": "Dr Samuel Johnson"
        },
        {
            "name": "The Badwater basin, the lowest point in North America, is located in which US valley?",
            "answers": [
                "Sun Valley",
                "Death Valley",
                "Big Smoky Valley",
                "Hudson Valley"
            ],
            "correct": "Death Valley"
        },
        {
            "name": "The Big Smoky Valley, that runs between the Toiyabe Range and the Toquima Range, is located in which US state?",
            "answers": [
                "Minnesota",
                "Indiana",
                "Oregon",
                "Nevada"
            ],
            "correct": "Nevada"
        },
        {
            "name": "The Grand Valley is an extended populated agricultural valley located along which US river?",
            "answers": [
                "Alabama River",
                "Mississippi River",
                "Colorado River",
                "Missouri River"
            ],
            "correct": "Colorado River"
        },
        {
            "name": "Which scenic US valley was described by conservationist John Muir in the following way: (...) None can escape its charms. Its natural beauty cleans and warms like a fire, and you will be willing to stay forever in one place like a tree.?",
            "answers": [
                "Willamette Valley",
                "Yosemite Valley",
                "Sun Valley",
                "Yakima Valley"
            ],
            "correct": "Yosemite Valley"
        },
        {
            "name": "This densely populated valley in the state of Utah is enclosed by steep mountains in every direction except the northwest.",
            "answers": [
                "St. Lawrence Valley",
                "Livermore Valley",
                "Las Vegas Valley",
                "Salt Lake Valley"
            ],
            "correct": "Salt Lake Valley"
        },
        {
            "name": "The Shenandoah Valley in western Virginia derives its name from a Native American expression meaning what?",
            "answers": [
                "Beautiful Daughter of the Fields",
                "Beautiful Mother of the Fruits",
                "Beautiful Mother of the Land",
                "Beautiful Daughter of the Stars"
            ],
            "correct": "Beautiful Daughter of the Stars"
        },
        {
            "name": "The Sonoma Valley in California is famous in the USA for its long-lived and high quality wine industry.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The Menomonee Valley along the Menomonee River in Milwaukee, Wisconsin has what peculiar shape?",
            "answers": [
                "S-shape",
                "Y-shape",
                "U-shape",
                "V-shape"
            ],
            "correct": "U-shape"
        },
        {
            "name": "Owens Valley, the arid ranching valley of the Owens River, stretches for approximately 75 miles (120 km) on the territory of which US state?",
            "answers": [
                "California",
                "Arkansas",
                "Alabama",
                "Arizona"
            ],
            "correct": "California"
        },
        {
            "name": "According to the 2000 Census, what is the approximate population of The Bronx, New York?",
            "answers": [
                "530,000",
                "1.3 million",
                "930,000",
                "130,000"
            ],
            "correct": "1.3 million"
        },
        {
            "name": "Who was the first European to see The Bronx, NY?",
            "answers": [
                "Henry Hutchinson",
                "Jonas Bronck",
                "Henry Hudson",
                "Jonas Valerian"
            ],
            "correct": "Henry Hudson"
        },
        {
            "name": "Which one of these statements is true about The Bronx, NY?",
            "answers": [
                "The only borough with a zoo",
                "The only borough  that has no colleges",
                "The only borough that is not an island",
                "The highest  borough above sea level ."
            ],
            "correct": "The only borough that is not an island"
        },
        {
            "name": "In 1643, a Native American uprising killed this unauthorized Puritan preacher of a dissident church discussion group, and pioneer in Rhode Island and the Bronx.",
            "answers": [
                "Peter Cooper",
                "Richard Morris",
                "Anne Hutchinson",
                "Roger Williams"
            ],
            "correct": "Anne Hutchinson"
        },
        {
            "name": "This oldest house in The Bronx, build in 1748, is New York Citys first house museum, operated since 1897 as a public museum by The National Society of Colonial Dames in the State of New York.",
            "answers": [
                "Valerian House",
                "Van Cortlandt House",
                "Throgs Neck",
                "Fordam Arms"
            ],
            "correct": "Van Cortlandt House"
        },
        {
            "name": "In 1776, 400 British and Hessian troops land in The Bronx.  A group of 400 American troops held them off while Washington got the main army away safely.  What was this battle called ?",
            "answers": [
                "The Battle of Pells Point",
                "The Battle of Throgs Neck",
                "The Battle of Eastchester",
                "The Battle of The Bronx"
            ],
            "correct": "The Battle of Pells Point"
        },
        {
            "name": "The first college in The Bronx, known as this today, was built in 1884.",
            "answers": [
                "Mercy College",
                "Manhattan College",
                "Fordham University",
                "Lehman College"
            ],
            "correct": "Fordham University"
        },
        {
            "name": "Edgar Allen Poe wrote this famous poem in The Bronx in 1846.",
            "answers": [
                "Call Me Ishmal",
                "Our American War",
                "Annabel Lee",
                "Hiawatha"
            ],
            "correct": "Annabel Lee"
        },
        {
            "name": "In 1899, this  world famous institution was opened in The Bronx.",
            "answers": [
                "Columbia University",
                "The U.S. Olympic Hall of Fame",
                "The Bronx Zoo",
                "The American Hall of Fame"
            ],
            "correct": "The Bronx Zoo"
        },
        {
            "name": "This world famous building opened in The Bronx in 1923.",
            "answers": [
                "The Top of the Sixes",
                "Yankee Stadium",
                "The Louis Morris Building",
                "The Shlump Towers"
            ],
            "correct": "Yankee Stadium"
        },
        {
            "name": "What are the colors on the flag of  The Bronx, NY?",
            "answers": [
                "Red, White, Blue",
                "Blue, White, Green",
                "Blue, White, Orange",
                "Red, Green, Orange"
            ],
            "correct": "Blue, White, Orange"
        },
        {
            "name": "Little Rock is the capital and most populous city of this US state.",
            "answers": [
                "Arkansas",
                "Wichita",
                "Kansas",
                "Topeka"
            ],
            "correct": "Arkansas"
        },
        {
            "name": "What US state borders the states of Virginia, Kentucky, Ohio, Pennsylvania and Maryland?",
            "answers": [
                "Tennessee",
                "Virginia",
                "West Virginia",
                "North Carolina"
            ],
            "correct": "West Virginia"
        },
        {
            "name": "Which US state is nicknamed The Pine Tree State?",
            "answers": [
                "New Hampshire",
                "Missouri",
                "Massachusetts",
                "Maine"
            ],
            "correct": "Maine"
        },
        {
            "name": "The southern region of what US state is known as Little Egypt?",
            "answers": [
                "Virginia",
                "Illinois",
                "Ohio",
                "Maryland"
            ],
            "correct": "Illinois"
        },
        {
            "name": "The capital city of this US state is Trenton, and its largest city is Newark.",
            "answers": [
                "Montana",
                "New Hampshire",
                "New Mexico",
                "New Jersey"
            ],
            "correct": "New Jersey"
        },
        {
            "name": "The name of which American state means red people in the Choctaw language?",
            "answers": [
                "Ohio",
                "Oklahoma",
                "North Dakota",
                "Oregon"
            ],
            "correct": "Oklahoma"
        },
        {
            "name": "What US state is known as The Volunteer State?",
            "answers": [
                "Utah",
                "Delaware",
                "Tennessee",
                "Colorado"
            ],
            "correct": "Tennessee"
        },
        {
            "name": "The flag of what US state features a gold torch and nineteen stars on a blue rectangular field?",
            "answers": [
                "Illinois",
                "Idaho",
                "Indiana",
                "Iowa"
            ],
            "correct": "Indiana"
        },
        {
            "name": "What US state is known as the Peach State?",
            "answers": [
                "Louisiana",
                "Georgia",
                "Montana",
                "Connecticut"
            ],
            "correct": "Georgia"
        },
        {
            "name": "The name of this US state means flat water and is derived from the name of the Platte River that flows through the state.",
            "answers": [
                "Montana",
                "Nebraska",
                "Utah",
                "Oklahoma"
            ],
            "correct": "Nebraska"
        },
        {
            "name": "Which country is known as Österreich in their native language?",
            "answers": [
                "Bulgaria",
                "Germany",
                "Austria",
                "The Netherlands"
            ],
            "correct": "Austria"
        },
        {
            "name": "Which countrys capital city is considered de facto capital of the European Union?",
            "answers": [
                "UKs",
                "Hollands",
                "Belgiums",
                "Frances"
            ],
            "correct": "Belgiums"
        },
        {
            "name": "Which country has the river Danube as most of its northern border?",
            "answers": [
                "Austria",
                "Hungary",
                "Bulgaria",
                "Romania"
            ],
            "correct": "Bulgaria"
        },
        {
            "name": "Which EU member is divided into a Greek and a Turkish part?",
            "answers": [
                "Greece",
                "Slovenia",
                "Cyprus",
                "Bulgaria"
            ],
            "correct": "Cyprus"
        },
        {
            "name": "Which countrys first president was Václav Havel, a writer and dramatist?",
            "answers": [
                "Hungary",
                "Poland",
                "Czech Republic",
                "Slovakia"
            ],
            "correct": "Czech Republic"
        },
        {
            "name": "Which Nordic country was first to enter the European Union?",
            "answers": [
                "Denmark",
                "Sweden",
                "Finland",
                "Norway"
            ],
            "correct": "Denmark"
        },
        {
            "name": "Which European Union member is home to Santa Claus?",
            "answers": [
                "Finland",
                "Cyprus",
                "Turkey",
                "Sweden"
            ],
            "correct": "Finland"
        },
        {
            "name": "Which of the EU founding members is the country of Liberté, Égalité, Fraternité?",
            "answers": [
                "France",
                "UK",
                "Germany",
                "Italy"
            ],
            "correct": "France"
        },
        {
            "name": "Which European Union member has the greatest number of islands?",
            "answers": [
                "Greece",
                "Italy",
                "UK",
                "Denmark"
            ],
            "correct": "Greece"
        },
        {
            "name": "Which country is the only member of the European Union that is partly located in Africa?",
            "answers": [
                "France",
                "UK",
                "Spain",
                "Malta"
            ],
            "correct": "Spain"
        },
        {
            "name": "Which EU member has a significant part of its territory below sea level?",
            "answers": [
                "Belgium",
                "The Netherlands",
                "Poland",
                "Estonia"
            ],
            "correct": "The Netherlands"
        },
        {
            "name": "This European Union member was once known for huge migration of its people, mainly to the US and UK. Nowadays, however, it attracts many immigrants from new EU members.",
            "answers": [
                "Spain",
                "Ireland",
                "Italy",
                "Germany"
            ],
            "correct": "Ireland"
        },
        {
            "name": "Which EU member was the largest country in Europe in XIV century, but now has land area of just 65 000 sq. km?",
            "answers": [
                "Denmark",
                "Lithuania",
                "Austria",
                "Netherlands"
            ],
            "correct": "Lithuania"
        },
        {
            "name": "Which mini country was among the European Union founding members?",
            "answers": [
                "Luxembourg",
                "Malta",
                "Monaco",
                "Liechtenstien"
            ],
            "correct": "Luxembourg"
        },
        {
            "name": "In which European Union member the native people make up only 60% of the total population?",
            "answers": [
                "Poland",
                "Austria",
                "Latvia",
                "France"
            ],
            "correct": "Latvia"
        },
        {
            "name": "Which former British colony joined the European Union in 2004?",
            "answers": [
                "Gibraltar",
                "Ireland",
                "Syria",
                "Malta"
            ],
            "correct": "Malta"
        },
        {
            "name": "Which country is the biggest contributor to the European Union budget?",
            "answers": [
                "UK",
                "Germany",
                "Italy",
                "France"
            ],
            "correct": "Germany"
        },
        {
            "name": "Which country has twice rejected European Union membership in national referendums?",
            "answers": [
                "Poland",
                "Norway",
                "Turkey",
                "UK"
            ],
            "correct": "Norway"
        },
        {
            "name": "Which European Union member was the first country to reject communist rule and thus sparkle the liberation of so called Eastern Block?",
            "answers": [
                "Czech Republic",
                "Romania",
                "Germany",
                "Poland"
            ],
            "correct": "Poland"
        },
        {
            "name": "Where is the European Unions westernmost point not counting overseas territories?",
            "answers": [
                "In Spains Canary islands",
                "In Portugals Azores islands",
                "In Iceland",
                "In Ireland"
            ],
            "correct": "In Portugals Azores islands"
        },
        {
            "name": "Which EU member state was known to the ancient Greeks and Romans as Dacia?",
            "answers": [
                "Macedonia",
                "Bulgaria",
                "Romania",
                "Slovenia"
            ],
            "correct": "Romania"
        },
        {
            "name": "Which country is the youngest independent state to be a member of the European Union? (It has been an independent country only since 1993.)",
            "answers": [
                "Montenegro",
                "Slovenia",
                "Slovakia",
                "Cyprus"
            ],
            "correct": "Slovakia"
        },
        {
            "name": "In May 2004 the European Union accepted 10 new countries. Which of them was first to adopt the Euro as its national currency?",
            "answers": [
                "Slovenia",
                "Poland",
                "Slovakia",
                "Estonia"
            ],
            "correct": "Slovenia"
        },
        {
            "name": "Which country associated with banking is not a member of the European Union?",
            "answers": [
                "Luxembourg",
                "UK",
                "Austria",
                "Switzerland"
            ],
            "correct": "Switzerland"
        },
        {
            "name": "In which country is the European Union’s biggest lake located?",
            "answers": [
                "In Finland",
                "In Poland",
                "In Sweden",
                "In Hungary"
            ],
            "correct": "In Sweden"
        },
        {
            "name": "Which Muslim country was recognized as a candidate for accession in the EU on December 12th, 1999 at the Helsinki summit of the European Council?",
            "answers": [
                "Israel",
                "Morocco",
                "Syria",
                "Turkey"
            ],
            "correct": "Turkey"
        },
        {
            "name": "All but three European Union official languages are from the Indo-European family. They are Finnish, Estonian and which other language?",
            "answers": [
                "Greek",
                "Hungarian",
                "Polsih",
                "Bulgarian"
            ],
            "correct": "Hungarian"
        },
        {
            "name": "Most of the European Union members are also part of a Schengen Agreement that allows their citizens to travel without passports across the member states. Which EU member is outside of the Schengen group?",
            "answers": [
                "Lithuania",
                "UK",
                "Poland",
                "Denmark"
            ],
            "correct": "UK"
        },
        {
            "name": "Which member of the European Union completely surrounds two countries that are outside of the club?",
            "answers": [
                "Austria",
                "France",
                "Spain",
                "Italy"
            ],
            "correct": "Italy"
        },
        {
            "name": "The head of John the Baptist can be seen in the Amiens Cathedral.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Wat Chiang Man is a temple in Thailand. In it, fifteen of these animals represent a sea of unformed matter upon which the cosmos of the Chedi floats.",
            "answers": [
                "Fish",
                "Monkeys",
                "Birds",
                "Elephants"
            ],
            "correct": "Elephants"
        },
        {
            "name": "The Shaolin temples are perhaps the Buddhist monasteries best known in the West. The word Shaolin means this.",
            "answers": [
                "Blue Sky",
                "Silent Soul",
                "Young Forest",
                "Pure Water"
            ],
            "correct": "Young Forest"
        },
        {
            "name": "Which EU country or US state is not entirely situated on (an) island(s)?",
            "answers": [
                "UK",
                "Hawaii",
                "Rhode Island",
                "Cyprus"
            ],
            "correct": "Rhode Island"
        },
        {
            "name": "Which country or state was the first to be attacked in WWII?",
            "answers": [
                "Denmark",
                "Hawaii",
                "Poland",
                "US Virgin Islands"
            ],
            "correct": "Poland"
        },
        {
            "name": "Where was the colonial settlement New Amsterdam located?",
            "answers": [
                "UK",
                "New Jersey",
                "Holland",
                "The state of New York"
            ],
            "correct": "The state of New York"
        },
        {
            "name": "Which of these is the largest in terms of area?",
            "answers": [
                "Georgia",
                "Florida",
                "Germany",
                "Finland"
            ],
            "correct": "Germany"
        },
        {
            "name": "Which of these peaks is the highest?",
            "answers": [
                "Grossglockner (Austria)",
                "McKinley (Alaska)",
                "Mont Blanc (France)",
                "Mount Elbert (Colorado)"
            ],
            "correct": "McKinley (Alaska)"
        },
        {
            "name": "Which of the following metropolitan areas is the largest in terms of population?",
            "answers": [
                "London",
                "New York City",
                "Paris",
                "Los Angeles"
            ],
            "correct": "New York City"
        },
        {
            "name": "Which river, flowing entirely through a single country or state, is the longest?",
            "answers": [
                "Vistula (Poland)",
                "Loire (France)",
                "Iowa (Iowa)",
                "Illinois (Illinois)"
            ],
            "correct": "Vistula (Poland)"
        },
        {
            "name": "What is the location of the worlds largest particle physics laboratory, established in 1954?",
            "answers": [
                "In the EU",
                "Both have laboratories of equal size.",
                "In the US",
                "In neither of them"
            ],
            "correct": "In neither of them"
        },
        {
            "name": "Which of the following countries and states shares its name with its capital city?",
            "answers": [
                "New York",
                "Malta",
                "Luxembourg",
                "Oklahoma"
            ],
            "correct": "Luxembourg"
        },
        {
            "name": "Which of these cities was established first?",
            "answers": [
                "New York City (New York)",
                "Copenhagen (Denmark)",
                "Madrid (Spain)",
                "Boston (Massachusetts)"
            ],
            "correct": "Madrid (Spain)"
        },
        {
            "name": "Which of the following flags doesnt include a cross?",
            "answers": [
                "the flag of Mississippi",
                "the flag of the UK",
                "the flag of Denmark",
                "the flag of Alaska"
            ],
            "correct": "the flag of Alaska"
        },
        {
            "name": "Birmingham is the most populous city of which country or US state?",
            "answers": [
                "Austria",
                "Alabama",
                "the UK",
                "Utah"
            ],
            "correct": "Alabama"
        },
        {
            "name": "Which one of these is not a Greek Mediterranean island?",
            "answers": [
                "Minorca",
                "Lemnos",
                "Crete",
                "Santorini"
            ],
            "correct": "Minorca"
        },
        {
            "name": "Which one of these Mediterranean islands is the smallest in area?",
            "answers": [
                "Corsica",
                "Cyprus",
                "Crete",
                "Corfu"
            ],
            "correct": "Corfu"
        },
        {
            "name": "What two colors are featured on the flag and coat of arms of Malta, a country consisting of seven Mediterranean islands?",
            "answers": [
                "Green and black",
                "Yellow and purple",
                "White and red",
                "Blue and yellow"
            ],
            "correct": "White and red"
        },
        {
            "name": "Which one of these Mediterranean islands, belonging to Italy, is the largest in area?",
            "answers": [
                "Sardinia",
                "Procida",
                "Capri",
                "Ischia"
            ],
            "correct": "Sardinia"
        },
        {
            "name": "This island, on which the volcano Etna is situated, is the largest Mediterranean island.",
            "answers": [
                "Sicily",
                "Corsica",
                "Rhodes",
                "Sardinia"
            ],
            "correct": "Sicily"
        },
        {
            "name": "Which French Mediterranean island is the birthplace of Napoléon Bonaparte?",
            "answers": [
                "Cyprus",
                "Gozo",
                "Corsica",
                "Montecristo"
            ],
            "correct": "Corsica"
        },
        {
            "name": "On April 11, 2002, Al-Qaeda carried out a terrorist attack near a synagogue on which Tunisian island?",
            "answers": [
                "Djerba",
                "Yalikavak",
                "Salih Ada",
                "Kargi Adasi"
            ],
            "correct": "Djerba"
        },
        {
            "name": "Which one of these Mediterranean islands does not belong to the group of the Balearic Islands?",
            "answers": [
                "Ibiza",
                "Mallorca",
                "Capri",
                "Formentera"
            ],
            "correct": "Capri"
        },
        {
            "name": "Which of these Mediterranean islands belongs to Croatia and is home of the endangered Griffon Vulture?",
            "answers": [
                "Filfla",
                "Procida",
                "Cres",
                "Gorgona"
            ],
            "correct": "Cres"
        },
        {
            "name": "What motto do North Carolina license plates carry?",
            "answers": [
                "The Old North State",
                "First In Flight",
                "Nothing Finer",
                "Tarheel State"
            ],
            "correct": "First In Flight"
        },
        {
            "name": "North Carolina has been well represented on the American Idol TV series Which contestant from the state became Americas Idol in 2004?",
            "answers": [
                "Fantasia Barrino",
                "Kerrier Ann Pickler",
                "Clay Aiken",
                "Taylor Hicks"
            ],
            "correct": "Fantasia Barrino"
        },
        {
            "name": "In 1838 the Federal Indian Removal Policy was forced into effect by The Treaty of New Echota. Twenty Cherokee Indians, none elected officials of the Cherokee Nation, signed over all Cherokee Territory east of the Mississippi for five million dollars. The Cherokee Indians were then forced to march from their home in the mountain of North Carolina to Okalahoma. This journey became know by what name?",
            "answers": [
                "Journey of No Return",
                "Trail of Tears",
                "Cherokee Death March",
                "March of Sorrow"
            ],
            "correct": "Trail of Tears"
        },
        {
            "name": "This military complex in North Carolina is one of the largest in the world and home to the 82 Airborne, the Golden Knights, and U.S. Army Special Operations Command.",
            "answers": [
                "Fort Bragg",
                "Fort Campbell",
                "Fort Polk",
                "Fort Dix"
            ],
            "correct": "Fort Bragg"
        },
        {
            "name": "Riga is the capital of which Baltic country?",
            "answers": [
                "Estonia",
                "Lithuania",
                "Belarus",
                "Latvia"
            ],
            "correct": "Latvia"
        },
        {
            "name": "What country gained independence from the British Mandate for Palestine in 1948?",
            "answers": [
                "Syria",
                "Sierra Leone",
                "Jordan",
                "Israel"
            ],
            "correct": "Israel"
        },
        {
            "name": "What city is the capital of Madagascar?",
            "answers": [
                "Antananarivo",
                "Toamasina",
                "Harare",
                "Maputo"
            ],
            "correct": "Antananarivo"
        },
        {
            "name": "What river encountered during the Lewis and Clark expedition (1804-1806) flows directly into the Pacific Ocean between Washington and Oregon?",
            "answers": [
                "Missouri",
                "Snake",
                "Columbia",
                "Colorado"
            ],
            "correct": "Columbia"
        },
        {
            "name": "Which explorer was the first to reach the South Pole?",
            "answers": [
                "Richard Byrd",
                "Sir Robert Falcon Scott",
                "Roald Amundsen",
                "Sir Ernest Henry Shackleton"
            ],
            "correct": "Roald Amundsen"
        },
        {
            "name": "What sea captain claimed British possession of the eastern part of Australia in 1770 naming it New South Wales?",
            "answers": [
                "James Cook",
                "Horatio Nelson",
                "Cuthbert Collingwood",
                "Arthur Phillip"
            ],
            "correct": "James Cook"
        },
        {
            "name": "In what South American country was Che Ernesto Guevara born?",
            "answers": [
                "Paraguay",
                "Argentina",
                "Bolivia",
                "Suriname"
            ],
            "correct": "Argentina"
        },
        {
            "name": "Near what sea was the ancient city of Troy located ?",
            "answers": [
                "Aegean",
                "Mediterranean",
                "Ionian",
                "Adriatic"
            ],
            "correct": "Aegean"
        },
        {
            "name": "Which of these rivers runs through the Czech Republic and Germany?",
            "answers": [
                "Elbe River",
                "Rhine River",
                "Danube River",
                "Rhone River"
            ],
            "correct": "Elbe River"
        },
        {
            "name": "Name the European co-principality whose heads of state are the President of France and the Bishop of Urgel.",
            "answers": [
                "Andorra",
                "Vatican City",
                "San Marino",
                "Malta"
            ],
            "correct": "Andorra"
        },
        {
            "name": "After World War II, this country was split into two states which were not reunited until 1990.",
            "answers": [
                "France",
                "Spain",
                "Austria",
                "Germany"
            ],
            "correct": "Germany"
        },
        {
            "name": "Mount Erebus is a volcano located on this continent.",
            "answers": [
                "Antarctica",
                "Europe",
                "South America",
                "Asia"
            ],
            "correct": "Antarctica"
        },
        {
            "name": "Rastafarianism is a religion that venerates Haile Selassie as a god. Haile Selassie was the emperor of the third most populous country in Africa. Name that country.",
            "answers": [
                "Libya",
                "Ethiopia",
                "Mozambique",
                "Gabon"
            ],
            "correct": "Ethiopia"
        },
        {
            "name": "Name Asias most densely populated country which has about three million people and an area of less than 250 square miles (402 km²).",
            "answers": [
                "Hokkaido",
                "Taiwan",
                "Laos",
                "Singapore"
            ],
            "correct": "Singapore"
        },
        {
            "name": "In what city in Switzerland is the headquarters of the World Trade Organization (WTO) located?",
            "answers": [
                "Geneva",
                "Berne",
                "Born",
                "Bamako"
            ],
            "correct": "Geneva"
        },
        {
            "name": "What is a ridge of coral in the sea called?",
            "answers": [
                "Atoll",
                "Coral Reef",
                "None of these",
                "Coral Island"
            ],
            "correct": "Coral Reef"
        },
        {
            "name": "Jenin is the largest city in the West Bank.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "It has been found that the black mineral soil from the Dead Sea area of Israel helps maintain a good, young complexion. What is the name of the Israeli company that exports beauty products from the Dead Sea area?",
            "answers": [
                "Naot",
                "Kfar Gideon",
                "Rafaele",
                "Ahava"
            ],
            "correct": "Ahava"
        },
        {
            "name": "The Dead Sea Scrolls, found in caves at Qumran, are believed to have been made by a group of people called what?",
            "answers": [
                "BHais",
                "Sircurai",
                "Essenes",
                "Sarafai"
            ],
            "correct": "Essenes"
        },
        {
            "name": "It is practically impossible to drown unwillingly in the Dead Sea.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "To which countrys territory does the Dead Sea belong?",
            "answers": [
                "Egypt and Israel share ownership",
                "Israel and Jordan share ownership",
                "Israels territory",
                "Saudi Arabias territory"
            ],
            "correct": "Israel and Jordan share ownership"
        },
        {
            "name": "What is the average salinity of the Dead Sea?",
            "answers": [
                "11 %",
                "30 %",
                "49 %",
                "54 %"
            ],
            "correct": "30 %"
        },
        {
            "name": "What body of water feeds the Dead Sea?",
            "answers": [
                "The Euphrates River",
                "The Mediterranean Ocean",
                "The Red Sea",
                "The Jordan River"
            ],
            "correct": "The Jordan River"
        },
        {
            "name": "The Bible tells the story of a woman that was turned into a pillar of salt near the Dead Sea. Who was her famous husband?",
            "answers": [
                "Issac",
                "Abraham",
                "Pharoah",
                "Lot"
            ],
            "correct": "Lot"
        },
        {
            "name": "The Dead Sea, the lowest point on Earth, is how far below sea level?",
            "answers": [
                "349  meters",
                "418 meters",
                "517  meters",
                "2608 meters"
            ],
            "correct": "418 meters"
        },
        {
            "name": "What was the name of the great fortification built in 37-31 BC by Roman king Herod I on a high hill above the Dead Sea?",
            "answers": [
                "Masada",
                "Kadesh-Barnea",
                "Herods Citadel",
                "Vespasians Citadel"
            ],
            "correct": "Masada"
        },
        {
            "name": "The Dead Sea is part of the Great Rift Valley. How far does this valley extend in length?",
            "answers": [
                "6700 miles",
                "697 miles",
                "1697 miles",
                "3700 miles"
            ],
            "correct": "3700 miles"
        },
        {
            "name": "No animals or plants live in the Dead Sea .",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "What famous California prison was home to Charles Manson?",
            "answers": [
                "The Bastille",
                "Sing Sing",
                "Joliet",
                "Folsom"
            ],
            "correct": "Folsom"
        },
        {
            "name": "American politician John McCain was incarcerated at the infamous Hoa Loa Prison, ironically called Hanoi Hilton.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The Rosenbergs, who were convicted of conspiracy, were held at what prison?",
            "answers": [
                "Folsom",
                "Alcatraz",
                "Attica",
                "Sing Sing"
            ],
            "correct": "Sing Sing"
        },
        {
            "name": "The Tower of London was built primarily to function as a fortress, a royal palace and a prison.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Attica Prison was the locale of a 1975 riot that resulted in 38 deaths.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Nelsons Column, built to commemorate Admiral Horatio Nelsons death, is a monument in this famous London square.",
            "answers": [
                "Soho Square",
                "Leicester Square",
                "Piccadilly Circus",
                "Trafalgar Square"
            ],
            "correct": "Trafalgar Square"
        },
        {
            "name": "Tiananmen Square, which has been the site of a number of political events and student protests, is located in what Chinese city?",
            "answers": [
                "Beijing",
                "Shanghai",
                "Guangzhou",
                "Taipei"
            ],
            "correct": "Beijing"
        },
        {
            "name": "The Kremlin, a historic fortified complex at the heart of Moscow, overlooks this square to the east.",
            "answers": [
                "Black Square",
                "Azadi Square",
                "Palace Square",
                "Red Square"
            ],
            "correct": "Red Square"
        },
        {
            "name": "What iconic world landmark, sometimes called the Crossroads of the World, was formerly named Longacre Square?",
            "answers": [
                "Grosvenor Square",
                "Times Square",
                "Madison Square",
                "St Jamess Square"
            ],
            "correct": "Times Square"
        },
        {
            "name": "What famous square is located in Vatican City, the papal enclave within Rome?",
            "answers": [
                "Campo dei Fiori",
                "St. Pauls Square",
                "St. Peters Square",
                "Piazza San Marco"
            ],
            "correct": "St. Peters Square"
        },
        {
            "name": "Grand Place, which is known for its large flower carpet, is the main square of this capital European city.",
            "answers": [
                "Berlin",
                "Paris",
                "Strasbourg",
                "Brussels"
            ],
            "correct": "Brussels"
        },
        {
            "name": "Which square is not correctly matched with the capital city in which it is located?",
            "answers": [
                "Red Square - Moscow",
                "Senate Square - Helsinki",
                "Azadi Square - Karachi",
                "Leicester Square - London"
            ],
            "correct": "Azadi Square - Karachi"
        },
        {
            "name": "Piazza San Marco (St Marks Square), which Napoleon allegedly called The drawing room of Europe, is the principal square of this Italian city.",
            "answers": [
                "Florence",
                "Venice",
                "Rome",
                "Milan"
            ],
            "correct": "Venice"
        },
        {
            "name": "What city in Eastern Europe, often associated with the story of Dracula, is also famous for Revolution Square, which was the site of a 1989 revolt?",
            "answers": [
                "Bucharest",
                "Warsaw",
                "Budapest",
                "Prague"
            ],
            "correct": "Bucharest"
        },
        {
            "name": "What former president has addressed more than a million people on many occasions on Plaza de la Revolución in Havana?",
            "answers": [
                "Benito Mussolini",
                "Fidel Castro",
                "Salvador Allende",
                "Francisco Franco"
            ],
            "correct": "Fidel Castro"
        },
        {
            "name": "What is the name of the sea that borders the Netherlands?",
            "answers": [
                "Red Sea",
                "Black Sea",
                "North Sea",
                "Mediterranean Sea"
            ],
            "correct": "North Sea"
        },
        {
            "name": "What are the three colours of the national flag of the Netherlands from top to bottom?",
            "answers": [
                "Red, white, and blue",
                "Blue, yellow, and red",
                "Red, blue, and white",
                "Green, red, and white"
            ],
            "correct": "Red, white, and blue"
        },
        {
            "name": "Rotterdam, located in the province of South Holland, is a major Dutch port.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Gouda and Edam are famous types of Dutch cheeses.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What is the capital city of the Netherlands?",
            "answers": [
                "Amsterdam",
                "Amersfort",
                "The Hague",
                "Zwolle"
            ],
            "correct": "Amsterdam"
        },
        {
            "name": "What two countries border the Netherlands?",
            "answers": [
                "Germany and Lithuania",
                "Germany and France",
                "France and Belgium",
                "Belgium and Germany"
            ],
            "correct": "Belgium and Germany"
        },
        {
            "name": "The name “Holland” is equivalent to the name the Netherlands.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Dutch is the official language of the Netherlands. What is the second official language of the country?",
            "answers": [
                "Low Saxon",
                "Frisian",
                "French",
                "English"
            ],
            "correct": "Frisian"
        },
        {
            "name": "Demographically, the Netherlands matches which of this characteristics?",
            "answers": [
                "All of these",
                "Its population is first in average height in the world.",
                "It is one of most densely populated countries in the world.",
                "It is one of the most densely cabled countries in the world."
            ],
            "correct": "All of these"
        },
        {
            "name": "What Mediterranean island is divided between a Greek community and a Turkish community (although the latter is not internationally recognized as a separate entity)?",
            "answers": [
                "Cyprus",
                "Crete",
                "Corsica",
                "Malta"
            ],
            "correct": "Cyprus"
        },
        {
            "name": "What South American island is shared by Chile and Argentina?",
            "answers": [
                "Islas Malvinas",
                "Tierra Del Fuego",
                "Islas Galapagos",
                "Isla De Pascua"
            ],
            "correct": "Tierra Del Fuego"
        },
        {
            "name": "What island is shared by Indonesia and the nation of East Timor?",
            "answers": [
                "Timor",
                "New Guinea",
                "Borneo",
                "Flores"
            ],
            "correct": "Timor"
        },
        {
            "name": "What European island is partially occupied by a neighbouring island nation, which has led to centuries of dispute?",
            "answers": [
                "Sardinia",
                "Sicily",
                "Iceland",
                "Ireland"
            ],
            "correct": "Ireland"
        },
        {
            "name": "Which of these islands is shared by three countries?",
            "answers": [
                "Luzon",
                "Ireland",
                "Madagascar",
                "Borneo"
            ],
            "correct": "Borneo"
        },
        {
            "name": "What is the name of the island shared by Haiti and the Dominican Republic?",
            "answers": [
                "Ile dHaiti",
                "Hispaniola",
                "Isla Dominica",
                "Carib Island"
            ],
            "correct": "Hispaniola"
        },
        {
            "name": "What island is shared by the nations of Indonesia and Papua New Guinea?",
            "answers": [
                "Bougainville",
                "New Guinea",
                "Timor",
                "Borneo"
            ],
            "correct": "New Guinea"
        },
        {
            "name": "This tiny Caribbean island is shared by France and the Netherlands.",
            "answers": [
                "St. Lucia",
                "St. Mark",
                "St. Martin",
                "St. Bartholomew"
            ],
            "correct": "St. Martin"
        },
        {
            "name": "The tiny island of Märket / Märketin in the Baltic Sea is shared by these two nations.",
            "answers": [
                "Sweden and Denmark",
                "Finland and Estonia",
                "Finland and Russia",
                "Finland and Sweden"
            ],
            "correct": "Finland and Sweden"
        },
        {
            "name": "The island of Sebatik, located east of Borneo, is shared by Indonesia and what other nation?",
            "answers": [
                "The Philippines",
                "Singapore",
                "Malaysia",
                "Brunei"
            ],
            "correct": "Malaysia"
        },
        {
            "name": "The word ‘ocean’ is of Greek origin.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Which is the smallest ocean?",
            "answers": [
                "The Atlantic Ocean",
                "The Indian Ocean",
                "The Southern Ocean",
                "The Arctic Ocean"
            ],
            "correct": "The Arctic Ocean"
        },
        {
            "name": "What portion of the Earth’s surface do oceans cover?",
            "answers": [
                "More than two thirds",
                "One third",
                "Half of it",
                "One fourth"
            ],
            "correct": "More than two thirds"
        },
        {
            "name": "The Marianas Trench, the deepest location on the surface of the Earths crust, lies in this ocean.",
            "answers": [
                "The Pacific Ocean",
                "The Southern Ocean",
                "The Indian Ocean",
                "The Atlantic Ocean"
            ],
            "correct": "The Pacific Ocean"
        },
        {
            "name": "The Dead Sea is actually a lake.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The Pacific Ocean is the largest of the Earths oceanic divisions. Its name is derived from the Latin name Mare Pacificum which has the following meaning.",
            "answers": [
                "The Peaceful Sea",
                "The Lonely Sea",
                "The Smooth Sea",
                "The Lonely Sea"
            ],
            "correct": "The Peaceful Sea"
        },
        {
            "name": "Which one of these Asian rivers does not flow into the Indian Ocean?",
            "answers": [
                "Zambezi",
                "Saigon",
                "Indus",
                "Ganges"
            ],
            "correct": "Saigon"
        },
        {
            "name": "What sea is known to be the only sea without shores?",
            "answers": [
                "The Sargasso Sea",
                "The Norwegian Sea",
                "The Labrador Sea",
                "The Celtic Sea"
            ],
            "correct": "The Sargasso Sea"
        },
        {
            "name": "The Bering Sea is a part of which ocean?",
            "answers": [
                "The Arctic Ocean",
                "The Pacific Ocean",
                "The Indian Ocean",
                "The Atlantic Ocean"
            ],
            "correct": "The Pacific Ocean"
        },
        {
            "name": "The Mediterranean Sea is connected to the Atlantic Ocean by this strait.",
            "answers": [
                "The Strait of Gibraltar",
                "The Dardanelles",
                "Strait of Magellan",
                "Strait of Dover"
            ],
            "correct": "The Strait of Gibraltar"
        },
        {
            "name": "The famous Île Notre-Dame, an artificial island built from earth excavated for a metropolitan railway, is located in which country?",
            "answers": [
                "England",
                "Canada",
                "France",
                "Switzerland"
            ],
            "correct": "Canada"
        },
        {
            "name": "What is the popular name of the artificial island in Tokyo Bay, whose name is Yume No Shima in Japanese?",
            "answers": [
                "Dream Island",
                "Water Lilly",
                "Wind Island",
                "Sun Island"
            ],
            "correct": "Dream Island"
        },
        {
            "name": "What is the name of the Peruvian floating islands, located in Lake Titicaca?",
            "answers": [
                "Chaka",
                "Laka",
                "Uros",
                "Barro"
            ],
            "correct": "Uros"
        },
        {
            "name": "Which man-made island, located to the southwest of Singapore, was created by joining several natural islands?",
            "answers": [
                "Formoza",
                "Jurong Island",
                "Spiral Island",
                "Neeltje-Jans"
            ],
            "correct": "Jurong Island"
        },
        {
            "name": "What is the name of the artificial island located in the Gatun Lake portion of the Panama Canal?",
            "answers": [
                "Thilafushi",
                "Shui Keng Teng",
                "Chubu",
                "Barro Colorado Island"
            ],
            "correct": "Barro Colorado Island"
        },
        {
            "name": "Which 20-kilometre artificial island, situated in the river Danube, was nicknamed the Copa Kagrana?",
            "answers": [
                "Peberholm",
                "Burj al-Arab",
                "Odaiba",
                "Donauinsel"
            ],
            "correct": "Donauinsel"
        },
        {
            "name": "Rokko island, constructed between 1973 and 1992, is an artificial island situated in which country?",
            "answers": [
                "France",
                "China",
                "The Netherlands",
                "Japan"
            ],
            "correct": "Japan"
        },
        {
            "name": "Which artificial island is situated in San Francisco Bay, between San Francisco and Oakland?",
            "answers": [
                "Hulhumalé",
                "Treasure Island",
                "Port Island",
                "Flakfortet"
            ],
            "correct": "Treasure Island"
        },
        {
            "name": "This artificial island, whose name literally means Pepper Islet, is part of the Oresund Bridge, connecting Denmark with Sweden.",
            "answers": [
                "Peberholm",
                "Kansai",
                "Flakfortet",
                "Palm Islands"
            ],
            "correct": "Peberholm"
        },
        {
            "name": "Spiral Island, a small Mexican artificial floating island, was situated on the coast of which sea?",
            "answers": [
                "The Black Sea",
                "The Caribbean Sea",
                "The Aegean Sea",
                "The North Sea"
            ],
            "correct": "The Caribbean Sea"
        },
        {
            "name": "The history of EU began on 25th of March 1957 when the European Economic Community was formed. What is the founding act called?",
            "answers": [
                "Treaty of Paris",
                "Schengen Agreement",
                "Schuman Declaration",
                "Treaty of Rome"
            ],
            "correct": "Treaty of Rome"
        },
        {
            "name": "Which country was not among the founding states of the European Economic Community (the European Union predecessor)?",
            "answers": [
                "Luxembourg",
                "Italy",
                "France",
                "UK"
            ],
            "correct": "UK"
        },
        {
            "name": "When did Ireland join the European Union?",
            "answers": [
                "1995",
                "1973",
                "1960",
                "1986"
            ],
            "correct": "1973"
        },
        {
            "name": "The citizens of which country rejected the idea of joining the EU in 1972 and again in 1994?",
            "answers": [
                "Norway",
                "Denmark",
                "Iceland",
                "Finland"
            ],
            "correct": "Norway"
        },
        {
            "name": "The EU has been a western club for most of its history. But which of these western countries has stayed out of the EU?",
            "answers": [
                "Belgium",
                "Spain",
                "Switzerland",
                "Luxembourg"
            ],
            "correct": "Switzerland"
        },
        {
            "name": "Which former Yugoslavian republic was the first to be admitted to the European Union?",
            "answers": [
                "Macedonia",
                "Serbia",
                "Croatia",
                "Slovenia"
            ],
            "correct": "Slovenia"
        },
        {
            "name": "The 6th European Union enlargement took place on May 1 2004. How many countries were admitted then?",
            "answers": [
                "8",
                "12",
                "10",
                "14"
            ],
            "correct": "10"
        },
        {
            "name": "Many countries held national referenda before joining the European Union. Which of the following nations showed the highest support for joining (92% in favour)?",
            "answers": [
                "Poland",
                "Ireland",
                "Lithuania",
                "Slovakia"
            ],
            "correct": "Slovakia"
        },
        {
            "name": "What kind of agreement is the Schengen Agreement?",
            "answers": [
                "It is a visionary chapter about a further EU enlargement.",
                "It allows EU citizens to travel without passports within the EU borders.",
                "It regulates the relationships with non-EU countries, especially Russia.",
                "It introduced a common currency - the Euro."
            ],
            "correct": "It allows EU citizens to travel without passports within the EU borders."
        },
        {
            "name": "When did the euro become the official currency of the Eurozone?",
            "answers": [
                "1999",
                "1994",
                "2002",
                "2004"
            ],
            "correct": "2002"
        },
        {
            "name": "The flag of which US state features a white American Bison on a blue field?",
            "answers": [
                "Nevada",
                "Wyoming",
                "Texas",
                "South Dakota"
            ],
            "correct": "Wyoming"
        },
        {
            "name": "The name of which US state means large creek in the Seneca language?",
            "answers": [
                "Idaho",
                "Ohio",
                "Texas",
                "Wisconsin"
            ],
            "correct": "Ohio"
        },
        {
            "name": "Montgomery is the capital city of this US state, and its largest city is Birmingham.",
            "answers": [
                "Arizona",
                "Alaska",
                "Arkansas",
                "Alabama"
            ],
            "correct": "Alabama"
        },
        {
            "name": "Everglades National Park is located in this US state.",
            "answers": [
                "Hawaii",
                "Nevada",
                "Georgia",
                "Florida"
            ],
            "correct": "Florida"
        },
        {
            "name": "The state drink of this US state is milk, its state bird is the Western Meadowlark, its state flower is the Wild Prairie Rose, and its state slogan is Legendary.",
            "answers": [
                "North Dakota",
                "South Dakota",
                "North Carolina",
                "South Carolina"
            ],
            "correct": "North Dakota"
        },
        {
            "name": "According to one theory, the name of this US state comes from the French word for hurricane.",
            "answers": [
                "Arizona",
                "Oregon",
                "Colorado",
                "Vermont"
            ],
            "correct": "Oregon"
        },
        {
            "name": "Baton Rouge is the capital of what US state?",
            "answers": [
                "New Hampshire",
                "Maine",
                "Louisiana",
                "North Carolina"
            ],
            "correct": "Louisiana"
        },
        {
            "name": "The largest city in what US state hosts Summerfest, or The Worlds Largest Music Festival?",
            "answers": [
                "New Jersey",
                "Wisconsin",
                "Kentucky",
                "Maryland"
            ],
            "correct": "Wisconsin"
        },
        {
            "name": "The name of this US state means great river and its symbol is the magnolia tree.",
            "answers": [
                "Missouri",
                "Massachusetts",
                "Mississippi",
                "Minnesota"
            ],
            "correct": "Mississippi"
        },
        {
            "name": "Which US state has been nicknamed The First State, The Small Wonder, Blue Hen State, and The Diamond State?",
            "answers": [
                "Pennsylvania",
                "Virginia",
                "Delaware",
                "New York"
            ],
            "correct": "Delaware"
        },
        {
            "name": "The smallest monkey on earth is found in the Philippines.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The Ifugao Rice Terraces are  located in Luzon, in the Philippines. If it were possible to place them end to  end, how long would they extend?",
            "answers": [
                "1,500 miles",
                "5,800 miles",
                "8,000 miles",
                "1,400 miles"
            ],
            "correct": "1,400 miles"
        },
        {
            "name": "What are the largest cities in the Philippines in terms of land area?",
            "answers": [
                "Manila and Davao City",
                "Zamballes and Puerto Princesa City",
                "Zamballes and Manila",
                "Davao City and Puerto Princesa City"
            ],
            "correct": "Davao City and Puerto Princesa City"
        },
        {
            "name": "The Philippines has the worlds largest deposit of heavy water which can be used for atomic energy, in spectroscopy, and as an automobile fuel. What makes heavy water different from water?",
            "answers": [
                "An extra hydrogen molecule",
                "One  less hydrogen molecule making it HO not H2O",
                "An extra oxygen molecule",
                "The presence of deuterium"
            ],
            "correct": "The presence of deuterium"
        },
        {
            "name": "From 1888-1889, American traders Simon and Thomas Metcalfe brought some men from Manila to the Western Hemisphere. Which state did they bring them to?",
            "answers": [
                "California",
                "Mexico",
                "Oregon",
                "Alaska"
            ],
            "correct": "Alaska"
        },
        {
            "name": "Who was the first  female  President of the Philippines?",
            "answers": [
                "Eleanor Concepcion",
                "Louiza Rizal",
                "Ramona  Magsaysay",
                "Corazon Aquino"
            ],
            "correct": "Corazon Aquino"
        },
        {
            "name": "Who is the first Filipino-American to become Governor of a US state?",
            "answers": [
                "Franklin  Rakata",
                "Jose Nisperos",
                "Jose Rizal",
                "Benjamin Cayetano"
            ],
            "correct": "Benjamin Cayetano"
        },
        {
            "name": "On December 7, 1941 the Japanese launched a sneak attack on Pearl Harbor. On that day they also launched a similar attack on this American airbase in the Philippines.",
            "answers": [
                "Subic Bay, Luzon, Philippines",
                "Capas Air Base,Tarlac, Philippines",
                "Angeles, Pampanga, Philippines",
                "Clark Air Force Base, Pampanga, Philippines"
            ],
            "correct": "Angeles, Pampanga, Philippines"
        },
        {
            "name": "Which best  describes the religious distribution of the populace of the Philippines?",
            "answers": [
                "82% Catholic, 9% Protestant, 5% Muslim and  3 % Buddhist",
                "92 % Catholic, 7 % Muslim, 1% all others",
                "57 % Muslim, 40 % Protestant, 1 % Catholic",
                "41% Catholic, 38 % Muslim, 10% Buddhist , 9 % Protestant"
            ],
            "correct": "82% Catholic, 9% Protestant, 5% Muslim and  3 % Buddhist"
        },
        {
            "name": "This US President was born in South Carolina although he lived in Tennessee when he became president.",
            "answers": [
                "William Henry Harrison",
                "Zachary Taylor",
                "Andrew Jackson",
                "Jimmy Carter"
            ],
            "correct": "Andrew Jackson"
        },
        {
            "name": "Which of the following states borders South Carolina?",
            "answers": [
                "Alabama",
                "Tennessee",
                "Virginia",
                "Georgia"
            ],
            "correct": "Georgia"
        },
        {
            "name": "In 1860 South Carolina became the first state to secede from the Union.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "South Carolina is the state whose shape most closely resembles a triangle.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What South Carolina Senator lived to be one hundred years old while still holding office?",
            "answers": [
                "Strom Thurmond",
                "Mike Mansfield",
                "James, (Jimmy)  Byrnes",
                "Fritz Hollings"
            ],
            "correct": "Strom Thurmond"
        },
        {
            "name": "Francis Marion fought the British in the swamps of South Carolina during the Revolutionary War, using guerrilla tactics, that the communist Vietcong in Vietnam would have understood well. For this Marion was given the following nickname.",
            "answers": [
                "The Swamp Fox",
                "The Yankee Ghost",
                "The Carolina Gamecock",
                "The Fighting Gator"
            ],
            "correct": "The Swamp Fox"
        },
        {
            "name": "The Grand Strand is a long stretch of sandy beach situated near which of the following?",
            "answers": [
                "Hilton Head",
                "Follie Beach",
                "Myrtle Beach",
                "Charleston"
            ],
            "correct": "Myrtle Beach"
        },
        {
            "name": "The city of Fargo is the largest city in which of these U.S. states?",
            "answers": [
                "North Carolina",
                "South Dakota",
                "North Dakota",
                "South Carolina"
            ],
            "correct": "North Dakota"
        },
        {
            "name": "Name the capital city of the U.S. state of Washington.",
            "answers": [
                "Seattle",
                "Olympia",
                "Madison",
                "Boise"
            ],
            "correct": "Olympia"
        },
        {
            "name": "Which U.S. state has been nicknamed The Cornhusker State?",
            "answers": [
                "Nevada",
                "Ohio",
                "Nebraska",
                "Wyoming"
            ],
            "correct": "Nebraska"
        },
        {
            "name": "Topeka is the capital city of which U.S. state?",
            "answers": [
                "Kansas",
                "Arkansas",
                "Missouri",
                "Colorado"
            ],
            "correct": "Kansas"
        },
        {
            "name": "Which of these nicknames refers to the U.S. state of Idaho?",
            "answers": [
                "Emerald State",
                "Pearl State",
                "Gem State",
                "Golden State"
            ],
            "correct": "Gem State"
        },
        {
            "name": "What does the name of the U.S. state of Utah mean in the Ute language?",
            "answers": [
                "Spirits of the rivers",
                "Children of the winds",
                "People of the mountains",
                "Land around the great lake"
            ],
            "correct": "People of the mountains"
        },
        {
            "name": "Saint Paul is the capital city of what US state?",
            "answers": [
                "Missouri",
                "Minnesota",
                "Mississippi",
                "None of these"
            ],
            "correct": "Minnesota"
        },
        {
            "name": "The Black Sea drains into the Atlantic Ocean via the Sea of Marmara and the Mediterranean Sea on the South West. On the other, North East side of the Black Sea, there is a small sea which is connected to the Black Sea via the Strait of Kerch. Name this sea.",
            "answers": [
                "The White Sea",
                "The Caspian Sea",
                "The Aral Sea",
                "The Sea of Azov"
            ],
            "correct": "The Sea of Azov"
        },
        {
            "name": "Six countries are located along the Black Sea border. Which one among them has the longest Black Sea coast line (not including the Sea of Azov coast line)?",
            "answers": [
                "Romania",
                "Turkey",
                "Russia",
                "Ukraine"
            ],
            "correct": "Turkey"
        },
        {
            "name": "Not counting Istanbul, which of these cities is located on Bosphorus on the side of the Sea of Marmara and is the biggest city on the coast of the Black Sea?",
            "answers": [
                "Trabzon, Turkey",
                "Sukhumi, Georgia",
                "Varna, Bulgaria",
                "Odessa, Ukraine"
            ],
            "correct": "Odessa, Ukraine"
        },
        {
            "name": "The names of three large rivers flowing into the Black Sea begin with a letter D. Danube, the largest of the three, flows through ten European countries and enters the Black Sea in Romania. The other two end their flow in this country.",
            "answers": [
                "Bulgaria",
                "Turkey",
                "Ukraine",
                "Russia"
            ],
            "correct": "Ukraine"
        },
        {
            "name": "The Black Sea city of Sukhumi is the capital of this republic that proclaimed its independence from Georgia in the 90s.",
            "answers": [
                "Ajaria",
                "Iberia",
                "Ossetia",
                "Abkhazia"
            ],
            "correct": "Abkhazia"
        },
        {
            "name": "The Black Sea, whose waters are known to be meromictic, is the largest water basin in the world where water layers do not intermix, and, thus, lack the dissolved form of what?",
            "answers": [
                "Salt",
                "Nitrogen",
                "Oxygen",
                "Hydrogen Sulfide"
            ],
            "correct": "Oxygen"
        },
        {
            "name": "The Crimean peninsula has been under the control of multiple powers over the centuries. Although it is considered to be an autonomous republic, it is a part of this nation.",
            "answers": [
                "Ukraine",
                "Romania",
                "Bulgaria",
                "Russia"
            ],
            "correct": "Ukraine"
        },
        {
            "name": "From the viewpoint of physical geography and ecology, the Ukrainian territories to the north of the Black Sea are known to contain what?",
            "answers": [
                "Low hills and forests",
                "Steppes or plains",
                "Mountains",
                "Sand desert"
            ],
            "correct": "Steppes or plains"
        },
        {
            "name": "The Black Sea reaches the maximum depth of 2,210 m. Its neighbor, the Sea of Azov, is unique in terms of sea depth for what reason?",
            "answers": [
                "Its floor sinks constantly because it is located on a large basalt plate.",
                "It is the shallowest sea in the world.",
                "It has almost perfectly flat sea floor with a depth of 1370 m.",
                "It has no established sea floor, it changes constantly with currents."
            ],
            "correct": "It is the shallowest sea in the world."
        },
        {
            "name": "All these entities are located around the Black Sea except for one. Name the exception.",
            "answers": [
                "Ural mountains",
                "Balkan peninsula",
                "Caucasus mountains",
                "Anatolia"
            ],
            "correct": "Ural mountains"
        },
        {
            "name": "The summit ridge of Mount Everest marks the border between which two Asian countries?",
            "answers": [
                "India and China",
                "Nepal and India",
                "China and Georgia",
                "China and Nepal"
            ],
            "correct": "China and Nepal"
        },
        {
            "name": "The locals of Nepal call the highest mount in the world - Mount Everest -  Sagarmatha. What is the meaning of this name in Sanskrit?",
            "answers": [
                "Holy Mountain",
                "Ice Zone",
                "Leader Among the Others",
                "Head of the Sky"
            ],
            "correct": "Head of the Sky"
        },
        {
            "name": "The highest mountain in the world is named after Sir George Everest, a famous Welshman. What was his profession?",
            "answers": [
                "Defender of human rights",
                "Actor",
                "Surveyor and geographer",
                "President"
            ],
            "correct": "Surveyor and geographer"
        },
        {
            "name": "The Himalayan range, part of which is Mount Everest, stretches across five different countries in Asia. Which of the following countries is not among them?",
            "answers": [
                "Israel",
                "Bhutan",
                "Pakistan",
                "India"
            ],
            "correct": "Israel"
        },
        {
            "name": "Edmund Hillary, the first mountaineer to reach mount Everest, was from what country?",
            "answers": [
                "The USA",
                "Great Britain",
                "Norway",
                "New Zealand"
            ],
            "correct": "New Zealand"
        },
        {
            "name": "What nationality was mathematician and surveyor Radhanath Sikdar, who was the first person to measure the height of Everest?",
            "answers": [
                "Bengali-Indian",
                "Chinese",
                "Pakistani",
                "Tibetan"
            ],
            "correct": "Bengali-Indian"
        },
        {
            "name": "Which of the following paths is one of the two most popular climbing routes to Mount Everest?",
            "answers": [
                "The south ridge from Tibet",
                "The east ridge from Pakistan",
                "The west ridge from India",
                "The southeast ridge from Nepal"
            ],
            "correct": "The southeast ridge from Nepal"
        },
        {
            "name": "What nationality was Junko Tabei - the first woman to reach the peak of Mount Everest?",
            "answers": [
                "Japanese",
                "Chinese",
                "Indian",
                "Nepalese"
            ],
            "correct": "Japanese"
        },
        {
            "name": "This Italian climber, who was in the first expedition to reach Mount Everest without oxygen tanks, later became the first person to climb the summit alone.",
            "answers": [
                "Reinhold Messner",
                "Laurie Skreslet",
                "Jerzy Kukuczka",
                "Tim Macartney"
            ],
            "correct": "Reinhold Messner"
        },
        {
            "name": "The term death zone refers to high altitudes, where the amount of oxygen cannot sustain human life and altitude acclimatization becomes impossible.  While this definition still applies, mount Everests death zone is significantly more difficult to survive.  Why?",
            "answers": [
                "Visibility is decreased due to constant snowstorms",
                "Extremely low temperatures result in quick freezing of body parts",
                "High wind velocity causes deadly avalanches",
                "Polar bears pose a serious threat to climbers"
            ],
            "correct": "Extremely low temperatures result in quick freezing of body parts"
        },
        {
            "name": "What is the largest island on the planet in terms of territory?",
            "answers": [
                "Madagascar",
                "Sumatra",
                "Greenland",
                "New Guinea"
            ],
            "correct": "Greenland"
        },
        {
            "name": "This island, located north of Australia, is the second-largest island on the planet.",
            "answers": [
                "Tasmania",
                "New Guinea",
                "South Island",
                "Sri Lanka"
            ],
            "correct": "New Guinea"
        },
        {
            "name": "This is the third-largest island on the planet, and it is also known as Kalimantan.",
            "answers": [
                "Bali",
                "Borneo",
                "Taiwan",
                "Java"
            ],
            "correct": "Borneo"
        },
        {
            "name": "This is the fourth-largest island in the world, and it is home to 5% of the plant and animal species on the planet.",
            "answers": [
                "Jamaica",
                "Madagascar",
                "Tasmania",
                "Cuba"
            ],
            "correct": "Madagascar"
        },
        {
            "name": "This is the largest European island, and the ninth-largest island on the planet.",
            "answers": [
                "Sicily",
                "Sardinia",
                "None of these",
                "Cyprus"
            ],
            "correct": "None of these"
        },
        {
            "name": "What is the largest island of Canada, and the fifth-largest island in the world?",
            "answers": [
                "Victoria Island",
                "Baffin Island",
                "Banks Island",
                "Devon Island"
            ],
            "correct": "Baffin Island"
        },
        {
            "name": "What is the most populous island in the world?",
            "answers": [
                "Java",
                "Cuba",
                "Borneo",
                "Honshu"
            ],
            "correct": "Java"
        },
        {
            "name": "This island is located in the Indian Ocean and is also known as Ceylon.",
            "answers": [
                "Borneo",
                "Taiwan",
                "Sumatra",
                "Sri Lanka"
            ],
            "correct": "Sri Lanka"
        },
        {
            "name": "The capital of this island country is Kingston.",
            "answers": [
                "Jamaica",
                "Haiti",
                "Hispaniola",
                "Cuba"
            ],
            "correct": "Jamaica"
        },
        {
            "name": "What is the largest island in the Mediterranean Sea?",
            "answers": [
                "Crete",
                "Corsica",
                "Sicily",
                "Cyprus"
            ],
            "correct": "Sicily"
        },
        {
            "name": "The geographic North and South Poles are at fixed locations and have not moved.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Which nation claims ownership of the South Pole?",
            "answers": [
                "Russia",
                "No nation does.",
                "France",
                "The USA"
            ],
            "correct": "No nation does."
        },
        {
            "name": "In 1728, he became the first person to sail into the Arctic Ocean and proved that Asia and North America are not joined by land.",
            "answers": [
                "Cook",
                "Bering",
                "Scott",
                "Peary"
            ],
            "correct": "Bering"
        },
        {
            "name": "Who is given credit  for being the first person to reach the North Pole?",
            "answers": [
                "Peary",
                "Bering",
                "Amundsen",
                "Scott"
            ],
            "correct": "Peary"
        },
        {
            "name": "What happened at the North Pole on August 3, 1958?",
            "answers": [
                "The northernmost US city, Naavetau, was founded.",
                "The first airport in the Arctic was built.",
                "A submarine travelled underwater under the Pole.",
                "The last Arctic Moa was killed."
            ],
            "correct": "A submarine travelled underwater under the Pole."
        },
        {
            "name": "The first time Antarctica was officially visited is a subject of controversy. Three people have claimed to have achieved this feat. Which of these is not one them?",
            "answers": [
                "Edward Bransfield",
                "Nathaniel B. Palmer",
                "Fabian von Bellinghausen",
                "James Cook"
            ],
            "correct": "James Cook"
        },
        {
            "name": "Which statement is true about the explorer and glaciologist Albert P. Crary?",
            "answers": [
                "He was the first person to set foot on Antarctica.",
                "He was the first pilot to fly across the South Pole.",
                "He was the first person to build a home above the Arctic Circle.",
                "He was the first person to reach both the North and South Poles."
            ],
            "correct": "He was the first person to reach both the North and South Poles."
        },
        {
            "name": "Who was the first person to reach the South Pole?",
            "answers": [
                "Scott",
                "Amundsen",
                "Henson",
                "Peary"
            ],
            "correct": "Amundsen"
        },
        {
            "name": "The temperature at the North Pole is higher than the temperature at the South Pole.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Few animals inhabit the North Pole. Which of these types of animals can never be seen there?",
            "answers": [
                "Penguins",
                "Bears",
                "Birds",
                "Fish"
            ],
            "correct": "Penguins"
        },
        {
            "name": "In 2000, microbes were found at the South Pole.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What river, draining into the Caspian Sea, is the longest in Europe?",
            "answers": [
                "Danube",
                "Ural",
                "Dnieper",
                "Volga"
            ],
            "correct": "Volga"
        },
        {
            "name": "Where are the origins of the Rhine?",
            "answers": [
                "Switzerland",
                "Austria",
                "Belgium",
                "Germany"
            ],
            "correct": "Switzerland"
        },
        {
            "name": "What is the longest river in France?",
            "answers": [
                "Loire",
                "Maine",
                "Odet",
                "Erdre"
            ],
            "correct": "Loire"
        },
        {
            "name": "What river rises in the northwestern Czech Republic, flows through Germany and into the North Sea?",
            "answers": [
                "Elbe",
                "Viskan",
                "Morava",
                "Oste"
            ],
            "correct": "Elbe"
        },
        {
            "name": "What river named after a mythological figure is the longest river of Great Britain?",
            "answers": [
                "Trent",
                "Great Ouse",
                "Thames",
                "Severn"
            ],
            "correct": "Severn"
        },
        {
            "name": "What river flows through ten European countries and is the longest in the European Union?",
            "answers": [
                "Rhine",
                "Dnieper",
                "Danube",
                "Elbe"
            ],
            "correct": "Danube"
        },
        {
            "name": "What river that flows through Spain and Portugal is the longest river on the Iberian Peninsula?",
            "answers": [
                "Guadiana",
                "Duero",
                "Guadalquivir",
                "Tagus"
            ],
            "correct": "Tagus"
        },
        {
            "name": "What is the longest river in Poland and the ninth longest in Europe?",
            "answers": [
                "Wda",
                "Nogat",
                "Vistula",
                "Dunajec"
            ],
            "correct": "Vistula"
        },
        {
            "name": "In what country does the River Po rise?",
            "answers": [
                "France",
                "Italy",
                "Belgium",
                "Switzerland"
            ],
            "correct": "Italy"
        },
        {
            "name": "This bridge, that spans the Tagus River, is the longest bridge in Europe.",
            "answers": [
                "Vizcaya Bridge",
                "Guadiana International Bridge",
                "Vasco da Gama Bridge",
                "Merida Bridge"
            ],
            "correct": "Vasco da Gama Bridge"
        },
        {
            "name": "How would you say What is your name? in Tagalog?",
            "answers": [
                "Ilang taong gulang ka na?",
                "Ano ang ginagawa mo?",
                "Ano ang pangalan mo?",
                "Saan ka nakatira?"
            ],
            "correct": "Ano ang pangalan mo?"
        },
        {
            "name": "The Filipino word Bulaklak means flower in English.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What does the Filipino word lungsod mean in English?",
            "answers": [
                "market",
                "city",
                "island",
                "province"
            ],
            "correct": "city"
        },
        {
            "name": "The Filipino word pangalan means name in English. What does the word pangngalan mean?",
            "answers": [
                "address",
                "adjective",
                "noun",
                "verb"
            ],
            "correct": "noun"
        },
        {
            "name": "The Filipino word maganda means beautiful in English.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What Filipino word means money in English?",
            "answers": [
                "tindahan",
                "mura",
                "pera",
                "wala"
            ],
            "correct": "pera"
        },
        {
            "name": "What does the Filipino word salita mean in English?",
            "answers": [
                "word",
                "phrase",
                "sentence",
                "meaning"
            ],
            "correct": "word"
        },
        {
            "name": "How will you say I Love You in Filipino?",
            "answers": [
                "Mahal Kita",
                "Ikaw Na Nga",
                "Sino Ka",
                "Hindi Ko Alam"
            ],
            "correct": "Mahal Kita"
        },
        {
            "name": "Who is considered the Father of Filipino Language?",
            "answers": [
                "Gloria Macapagal-Arroyo",
                "Ferdinand Marcos",
                "Emilio Aguinaldo",
                "Manuel Quezon"
            ],
            "correct": "Manuel Quezon"
        },
        {
            "name": "This western half of this island is part of Indonesia, while the eastern half is a nation which gained independence in 2002, following hundreds of years of European colonialism and 25 years of Indonesian occupation.",
            "answers": [
                "Borneo",
                "Sumatra",
                "Timor",
                "Celebes"
            ],
            "correct": "Timor"
        },
        {
            "name": "Which Indonesian island is shared by four Indonesian provinces and two other nations: the eastern half of Malaysia and the tiny oil-rich sultanate of Brunei?",
            "answers": [
                "New Guinea",
                "Sumatra",
                "Borneo",
                "Flores"
            ],
            "correct": "Borneo"
        },
        {
            "name": "These islands were the original spice islands, which have drawn traders from all over the world for thousands of years, and were a major factor in causing the European age of exploration, as explorers set out to find the fabled East Indies.",
            "answers": [
                "Ternate, Tidore and Banda",
                "Timor, Flores and Sumba",
                "Celebes and New Guinea",
                "Sumatra and Borneo"
            ],
            "correct": "Ternate, Tidore and Banda"
        },
        {
            "name": "Which Indonesian island was in the news in 2003 when the remains of an extinct variety of small hominids (popularly referred to as Hobbits), measuring about 3 feet in height, were found there?",
            "answers": [
                "Sumatra",
                "New Guinea",
                "Timor",
                "Flores"
            ],
            "correct": "Flores"
        },
        {
            "name": "This island, the second largest in the world, is shared by Indonesia and another country. Because of its impenetrable swamps, jungles and towering mountains, it wasnt explored by outsiders until the past 100 years, and new tribes of humans with stone-age societies are still being discovered in the interior.",
            "answers": [
                "New Guinea",
                "Flores",
                "Celebes",
                "Sumatra"
            ],
            "correct": "New Guinea"
        },
        {
            "name": "This island is the third largest in Indonesia, and is probably known for its coffee. It is also home to unique (and endangered) species of elephants, tigers, and rhinoceroses. It is also one of two islands which have indigenous orangutans.",
            "answers": [
                "Timor",
                "Sumatra",
                "New Guinea",
                "Celebes"
            ],
            "correct": "Sumatra"
        },
        {
            "name": "Which of these describes best the location of the volcanic island of Krakatau (sometimes erroneously called Krakatoa), which had a well-known and devastating eruption on August 23, 1883?",
            "answers": [
                "West of New Guinea",
                "East of Java",
                "Between Java and Sumatra",
                "Off the coast of Borneo"
            ],
            "correct": "Between Java and Sumatra"
        },
        {
            "name": "This volcano, located on the island of Sumbawa, erupted in 1815, generally regarded as the largest volcanic eruption in recorded history. The eruption killed thousands, and changed global climatic patterns.",
            "answers": [
                "Merapi",
                "Krakatau",
                "Mandiri",
                "Tambora"
            ],
            "correct": "Tambora"
        },
        {
            "name": "Wichita, the county seat of Sedgwick County, is nicknamed what?",
            "answers": [
                "Air Capital",
                "Mountain Capital",
                "Plain Capital",
                "River capital"
            ],
            "correct": "Air Capital"
        },
        {
            "name": "Wichita, a major aircraft manufacturing hub and cultural center, is the capital of Kansas.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "This multifunctional facility is located at 225 West Douglas, Wichita, Kansas.",
            "answers": [
                "Exploration Place",
                "None of these",
                "Century II Convention center",
                "Hyatt hotel"
            ],
            "correct": "Century II Convention center"
        },
        {
            "name": "McConnell Air Force Base is a United States Air Force base located in Sedgwick County, near Wichita, Kansas.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What is the name of the European peninsula on which Italy is located?",
            "answers": [
                "Scandinavian Peninsula",
                "Iberian Peninsula",
                "Italian Peninsula",
                "Balkan Peninsula"
            ],
            "correct": "Italian Peninsula"
        },
        {
            "name": "What is the capital and largest city of Italy?",
            "answers": [
                "Milan",
                "Genoa",
                "Rome",
                "Venice"
            ],
            "correct": "Rome"
        },
        {
            "name": "What large mountain range is located entirely within the territory of Italy?",
            "answers": [
                "Alps",
                "Andes",
                "Apennines",
                "Carpathians"
            ],
            "correct": "Apennines"
        },
        {
            "name": "Italy is a major world exporter of which of the following?",
            "answers": [
                "All of these",
                "Sport and luxury vehicles",
                "Motor vehicles",
                "Fashion items"
            ],
            "correct": "All of these"
        },
        {
            "name": "What is the meaning of the Italian word riavvolgere, from which the name of the popular Italian dish ravioli originated?",
            "answers": [
                "Poor peasant",
                "Onion",
                "To wrap",
                "To indulge"
            ],
            "correct": "To wrap"
        },
        {
            "name": "Which of these colors is not present on the national flag of the Italian Republic?",
            "answers": [
                "White",
                "Red",
                "Blue",
                "Green"
            ],
            "correct": "Blue"
        },
        {
            "name": "What is the title of the national anthem of the Italian Republic?",
            "answers": [
                "The Italian Anthem",
                "The Song of the Italians",
                "Italia, Land of Glory",
                "Italian Dream"
            ],
            "correct": "The Song of the Italians"
        },
        {
            "name": "What is the meaning of the name of Italy according to scientists who have studied its origin?",
            "answers": [
                "Land of the holy water",
                "Land of the lamb",
                "Land of young cattle",
                "Land of mountains"
            ],
            "correct": "Land of young cattle"
        },
        {
            "name": "What is the shape of the Italian Peninsula?",
            "answers": [
                "Skull",
                "Boot",
                "Dog",
                "Hand"
            ],
            "correct": "Boot"
        },
        {
            "name": "In which of these years did the Italian national football team win the FIFA World Cup?",
            "answers": [
                "1982",
                "2006",
                "All of these",
                "1938"
            ],
            "correct": "All of these"
        },
        {
            "name": "The first European to explore the Chesapeake Bay surrounded by modern day Maryland and Virginia was the Italian Giovanni da Verrazzano.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Maryland is well known as a colony of religious tolerance.  It was the first American colony to fully accept practitioners of which religion?",
            "answers": [
                "Quakers",
                "Presbyterians",
                "Catholics",
                "Jews"
            ],
            "correct": "Catholics"
        },
        {
            "name": "Maryland Day is celebrated every year on what date?",
            "answers": [
                "The first day of crab season",
                "Preakness Saturday",
                "March 25th",
                "March 1st"
            ],
            "correct": "March 25th"
        },
        {
            "name": "Which was the seat of colonial government in Maryland until 1708?",
            "answers": [
                "Annapolis",
                "Port Tobacco",
                "Baltimore",
                "St. Mary’s City"
            ],
            "correct": "St. Mary’s City"
        },
        {
            "name": "The Mason Dixon Line is a demarcation line surveyed between 1763 and 1767 by Charles Mason and Jeremiah Dixon to determine what?",
            "answers": [
                "The boundary between the confederacy and the union",
                "The boundary between the slave states and free states",
                "The boundary between Maryland and Pennsylvania",
                "The boundary between southern colonies and northern colonies"
            ],
            "correct": "The boundary between Maryland and Pennsylvania"
        },
        {
            "name": "What are Maryland’s two official state nicknames?",
            "answers": [
                "“The Old Line State” and “The Free State”",
                "“The Crab State” and “Our Chesapeake Heritage”",
                "“The Crab State” and “The Old Line State”",
                "“The Free State” and “Our Chesapeake Heritage”"
            ],
            "correct": "“The Old Line State” and “The Free State”"
        },
        {
            "name": "Maryland derives its name from its Catholic founders, who named the state after which historic person?",
            "answers": [
                "Mary, Queen of Scots",
                "Mary Magdalene",
                "The Virgin Mary",
                "Queen Henrietta Maria of France"
            ],
            "correct": "Queen Henrietta Maria of France"
        },
        {
            "name": "What is the state sport of Maryland?",
            "answers": [
                "Steeplechase",
                "Jousting",
                "Lacrosse",
                "Horse racing"
            ],
            "correct": "Jousting"
        },
        {
            "name": "Which of the following famous people is NOT a Marylander?",
            "answers": [
                "John Wilkes Booth",
                "Frederick Douglass",
                "Tom Clancy",
                "Valerie Bertinelli"
            ],
            "correct": "Valerie Bertinelli"
        },
        {
            "name": "Which is the state flower of Maryland?",
            "answers": [
                "Mountain Laurel",
                "Peach Blossom",
                "Black-eyed Susan",
                "Flowering Dogwood"
            ],
            "correct": "Black-eyed Susan"
        },
        {
            "name": "How tall is Mount Everest?",
            "answers": [
                "8,859 m",
                "8,849 m",
                "8,850 m",
                "8,840 m"
            ],
            "correct": "8,849 m"
        },
        {
            "name": "What is the Tibetan name of Mt. Everest?",
            "answers": [
                "Appa Sherpa",
                "Sagarmatha",
                "Chomolungma",
                "Temba Tsheri"
            ],
            "correct": "Chomolungma"
        },
        {
            "name": "The Sanskrit name of Mt. Everest is Devgiri.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The easier and more frequently-used of the two main routes for climbing Mount Everest is the Northeast Ridge (or North Col Route).",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "The term Death Zone refers to what altitude?",
            "answers": [
                "above 8,100 m",
                "above 8,000 m",
                "above 7,900 m",
                "above 7,500 m"
            ],
            "correct": "above 8,000 m"
        },
        {
            "name": "What is considered to be the final major hurdle before reaching the summit of Mt. Everest from the South Col Route?",
            "answers": [
                "Hillary Step",
                "First Step, Second Step and Third Step",
                "Kangshung face",
                "Lhotse face"
            ],
            "correct": "Hillary Step"
        },
        {
            "name": "Has anyone landed an aircraft or helicopter on the summit of Mt. Everest?",
            "answers": [
                "Yes",
                "No",
                null,
                null
            ],
            "correct": "Yes"
        },
        {
            "name": "Part of the Southeast Ridge Route for climbing Mt. Everest is the Lhotse Face in which Camp III is established. There are 14 peaks over 8,000 meters. Where does Lhotse rank?",
            "answers": [
                "4",
                "3",
                "8",
                "13"
            ],
            "correct": "4"
        },
        {
            "name": "Who is the first American to ascend all fourteen peaks over 8,000m without oxygen?",
            "answers": [
                "Reinhold Messner",
                "Peter Habeler",
                "Ed Viesturs",
                "George Mallory"
            ],
            "correct": "Ed Viesturs"
        },
        {
            "name": "During Roman occupation, this city, popular nowadays as the Heaven of Romance, was called Lutetia.",
            "answers": [
                "Venice",
                "Paris",
                "London",
                "Rome"
            ],
            "correct": "Paris"
        },
        {
            "name": "In what European city are Piazza San Marco and Canale Grande located?",
            "answers": [
                "Venice",
                "Lisbon",
                "Madrid",
                "Florence"
            ],
            "correct": "Venice"
        },
        {
            "name": "In what country will you find the impressive Neuschwanstein Castle, which was the inspiration for Sleeping Beauty Castle at Disneyland Park?",
            "answers": [
                "Denmark",
                "Austria",
                "Germany",
                "England"
            ],
            "correct": "Germany"
        },
        {
            "name": "What is the birthplace of Johann Strauss II, also known as The Waltz King?",
            "answers": [
                "Salzburg",
                "Vienna",
                "Berlin",
                "Prague"
            ],
            "correct": "Vienna"
        },
        {
            "name": "What popular tourist destination stands at the foot of the Maritime Alps?",
            "answers": [
                "Monte Carlo",
                "Verona",
                "Vatican City",
                "Lisbon"
            ],
            "correct": "Monte Carlo"
        },
        {
            "name": "This city, situated on the River Vltava, is sometimes called the mother of cities, the city of a hundred spires and the golden city.",
            "answers": [
                "Athens",
                "Rome",
                "Prague",
                "Bucharest"
            ],
            "correct": "Prague"
        },
        {
            "name": "What city is home of Broadway Theatre?",
            "answers": [
                "Washington",
                "Boston",
                "New Orleans",
                "New York"
            ],
            "correct": "New York"
        },
        {
            "name": "This island in the Pacific Ocean is surrounded by a lagoon and a barrier reef.",
            "answers": [
                "Bora Bora",
                "Cyprus",
                "Thasos",
                "Tupai"
            ],
            "correct": "Bora Bora"
        },
        {
            "name": "The city of Casablanca, which is the setting of the classic 1942 movie, starring Humphrey Bogart and Ingrid Bergman, is located in what African country?",
            "answers": [
                "Egypt",
                "Morocco",
                "Tunisia",
                "Algeria"
            ],
            "correct": "Morocco"
        },
        {
            "name": "What islands are knows as “The beautiful garland in the Indian Ocean”?",
            "answers": [
                "Aeolian Islands",
                "Pontine Islands",
                "Flegrean Islands",
                "Maldives Islands"
            ],
            "correct": "Maldives Islands"
        },
        {
            "name": "This island, located in the Caribbean Sea, is the largest island of an archipelago called Greater Antilles. The city of Havana is located on it.",
            "answers": [
                "Cuba",
                "Hispaniola",
                "Haiti",
                "Jamaica"
            ],
            "correct": "Cuba"
        },
        {
            "name": "This island, also known as Formosa, is the largest island of the Republic of China.",
            "answers": [
                "Matsu",
                "Taiwan",
                "Penghu",
                "Honshu"
            ],
            "correct": "Taiwan"
        },
        {
            "name": "What island, located in the North Pacific Ocean, is also known as the Big Island?",
            "answers": [
                "Cuba",
                "Haiti",
                "Hawaii",
                "Maui"
            ],
            "correct": "Hawaii"
        },
        {
            "name": "This island country whose capital is Nicosia is the third-largest Mediterranean island.",
            "answers": [
                "Cyprus",
                "Crete",
                "Malta",
                "Lesbos"
            ],
            "correct": "Cyprus"
        },
        {
            "name": "This island is one of Indonesias 33 provinces.",
            "answers": [
                "Java",
                "Sumatra",
                "Bali",
                "Lombok"
            ],
            "correct": "Bali"
        },
        {
            "name": "This European island is situated in the North Atlantic Ocean, south of the Arctic Circle. Its largest city is Reykjavik.",
            "answers": [
                "Iceland",
                "Ireland",
                "Greenland",
                "None of these"
            ],
            "correct": "Iceland"
        },
        {
            "name": "This is the sixth largest island in the world, and the largest island that is entirely in Indonesia.",
            "answers": [
                "Sumatra",
                "New Guinea",
                "Java",
                "Borneo"
            ],
            "correct": "Sumatra"
        },
        {
            "name": "This island, sometimes called island of inspiration, is part of a very large country, and has a unique fauna.",
            "answers": [
                "Hawaii",
                "Tasmania",
                "Madagascar",
                "Greenland"
            ],
            "correct": "Tasmania"
        },
        {
            "name": "Valletta is the capital city of this island country in the Mediterranean.",
            "answers": [
                "Majorca",
                "Malta",
                "Corsica",
                "Cyprus"
            ],
            "correct": "Malta"
        },
        {
            "name": "Name the largest of the Canary Islands.",
            "answers": [
                "Tenerife",
                "Gran Canaria",
                "La Palma",
                "Montana Clara"
            ],
            "correct": "Tenerife"
        },
        {
            "name": "What is the name of the capital and largest city of Spain?",
            "answers": [
                "Madrid",
                "Rome",
                "Milan",
                "Valencia"
            ],
            "correct": "Madrid"
        },
        {
            "name": "What is the meaning of the name of Spain according to a popular theory?",
            "answers": [
                "Land of rabbits",
                "All of these",
                "Edge",
                "Land of the setting sun"
            ],
            "correct": "All of these"
        },
        {
            "name": "Spain does not share a border with which one of these countries?",
            "answers": [
                "Germany",
                "Portugal",
                "France",
                "Andorra"
            ],
            "correct": "Germany"
        },
        {
            "name": "What is the official language spoken on the territory of Spain?",
            "answers": [
                "All of these",
                "Spanish and Galician",
                "Spanish and Basque",
                "Spanish"
            ],
            "correct": "Spanish"
        },
        {
            "name": "When is the national holiday called Hispanic Day celebrated annually in Spain?",
            "answers": [
                "October 12",
                "August 15",
                "December 6",
                "December 8"
            ],
            "correct": "October 12"
        },
        {
            "name": "What is the title of the national anthem of Spain?",
            "answers": [
                "My Precious Kingdom",
                "God Save Spain",
                "Proud Spain",
                "The Royal March"
            ],
            "correct": "The Royal March"
        },
        {
            "name": "What are the colors of the three vertical stripes on the national flag of Spain?",
            "answers": [
                "Blue, white, blue",
                "Yellow, red and black",
                "Red, yellow, red",
                "White, blue and red"
            ],
            "correct": "Red, yellow, red"
        },
        {
            "name": "What is the main ingredient of the traditional Spanish dish paella?",
            "answers": [
                "Lentils",
                "Rice",
                "Dough",
                "Potatoes"
            ],
            "correct": "Rice"
        },
        {
            "name": "Which of these popular musicians was not born in Spain?",
            "answers": [
                "Paco de Lucia",
                "Shakira",
                "Alejandro Sanz",
                "Enrique Iglesias"
            ],
            "correct": "Shakira"
        },
        {
            "name": "Spanish painter and sculptor Pablo Picasso is known for co-founding what art movement?",
            "answers": [
                "Cubism",
                "Expressionism",
                "Surrealism",
                "Neorealism"
            ],
            "correct": "Cubism"
        },
        {
            "name": "In which section of Israel is Eilat located?",
            "answers": [
                "East",
                "West",
                "South",
                "North"
            ],
            "correct": "South"
        },
        {
            "name": "When was the city of Eilat founded?",
            "answers": [
                "1951",
                "1939",
                "1941",
                "1929"
            ],
            "correct": "1951"
        },
        {
            "name": "What is the population of Eilat according to the Israel Central Bureau of Statistics end of 2004 census?",
            "answers": [
                "45,800",
                "179,400",
                "79,200",
                "129,700"
            ],
            "correct": "45,800"
        },
        {
            "name": "What best describes the climate of Eilat?",
            "answers": [
                "Hot and dry all year",
                "Humid and hot  summers and humid and cool winters",
                "Humid, hot summers and dry, cool winters",
                "Hot, dry summers and humid, cool winters"
            ],
            "correct": "Hot and dry all year"
        },
        {
            "name": "Which city is a sister city to Eilat?",
            "answers": [
                "Hollywood, Florida",
                "West Palm Beach, Florida",
                "Los Angeles, California",
                "Boca Raton, Florida"
            ],
            "correct": "Los Angeles, California"
        },
        {
            "name": "Which is the major source of revenue for the Eilat area?",
            "answers": [
                "Agriculture",
                "Tourism",
                "Jewelry",
                "Mining"
            ],
            "correct": "Tourism"
        },
        {
            "name": "Israels major airport is located near Eilat.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Which four countries can be seen from Eilat?",
            "answers": [
                "Israel, Egypt, Saudi Arabia, Yemen",
                "Israel, Egypt, Jordan, Saudi Arabia",
                "Israel, Jordan, Syria, Lebanon",
                "Israel, Egypt, Jordan, Syria"
            ],
            "correct": "Israel, Egypt, Jordan, Saudi Arabia"
        },
        {
            "name": "In 1967, Egypt effectively blockaded Eilat by closing this area to Israel.",
            "answers": [
                "The Golan Heights",
                "The Sinai Peninsula",
                "The Straits of Tiran",
                "The Suez Canal"
            ],
            "correct": "The Straits of Tiran"
        },
        {
            "name": "The Peace Treaty between Jordan and Israel was signed near Eilat. In what year did this happen?",
            "answers": [
                "October, 2004",
                "October,1968",
                "October, 1975",
                "October, 1994"
            ],
            "correct": "October, 1994"
        },
        {
            "name": "Where does the Freedom Trail begin and end?",
            "answers": [
                "Kings Chapel, Bunker Hill Monument",
                "Trinity Church, Long Wharf",
                "Boston Public Gardens, Old North Church",
                "Boston Common, Charlestown Navy Yard"
            ],
            "correct": "Boston Common, Charlestown Navy Yard"
        },
        {
            "name": "On October 24, 1901, Annie Edson Taylor became the first person to go over the Niagara Falls in a barrel as a publicity stunt. How old was she at the time?",
            "answers": [
                "63",
                "23",
                "43",
                "33"
            ],
            "correct": "63"
        },
        {
            "name": "What is the famous Niagara boat called?",
            "answers": [
                "Niagara-on-the-Lake",
                "Lady Horseshoe",
                "Maid of the Mist",
                "Lady of the Mist"
            ],
            "correct": "Maid of the Mist"
        },
        {
            "name": "Name the smallest of the three waterfalls that make up the Niagara Falls.",
            "answers": [
                "Bridal Veil Falls",
                "There are only two waterfalls that make up the Niagara Falls.",
                "Goat Falls",
                "Horseshoe"
            ],
            "correct": "Bridal Veil Falls"
        },
        {
            "name": "Approximately what percentage of the water of the Niagara River flows over the Horseshoe Falls?",
            "answers": [
                "70%",
                "90%",
                "50%",
                "10%"
            ],
            "correct": "90%"
        },
        {
            "name": "The flow of the water over the American Falls was completely blocked from June to November in what year?",
            "answers": [
                "1942",
                "1969",
                "1983",
                "1936"
            ],
            "correct": "1969"
        },
        {
            "name": "How long is the crest of the American Falls if measured along the jagged lip of the falls?",
            "answers": [
                "393  feet  (120 m)",
                "950  feet  (290 m)",
                "688  feet  (210 m)",
                "1310 feet  (400 m)"
            ],
            "correct": "950  feet  (290 m)"
        },
        {
            "name": "What event related to the Niagara Falls occurred on March 29th 1848?",
            "answers": [
                "The flow of water over both falls stopped to the point where people could walk around the riverbed.",
                "The flow of water was stopped completely over the American Falls.",
                "The waterfalls were named",
                "A third, much smaller waterfall was discovered."
            ],
            "correct": "The flow of water over both falls stopped to the point where people could walk around the riverbed."
        },
        {
            "name": "The cities of Niagara Falls, Ontario and Niagara Falls, New York are connected by three bridges. Which of them is the oldest one?",
            "answers": [
                "Whirlpool Rapids Bridge",
                "Rainbow Bridge",
                "Kingston Bridge",
                "Lewiston-Queenston Bridge"
            ],
            "correct": "Whirlpool Rapids Bridge"
        },
        {
            "name": "When was the Skylon Tower, Niagaras most famous landmark, built?",
            "answers": [
                "in the 1950s",
                "in the 1940s",
                "in the 1960s",
                "in the 1990s"
            ],
            "correct": "in the 1960s"
        },
        {
            "name": "Around the year 1859, Jean-François Blondin crossed the gorge below Niagara Falls on a tightrope 1100 feet (335 m) long, 160 feet (50 m) above the water. He made this accomplishment several times including which of the following theatrical variations?",
            "answers": [
                "All of these",
                "He carried his manager on his back.",
                "He sat down midway while he cooked and ate an omelette.",
                "He was blindfolded."
            ],
            "correct": "All of these"
        },
        {
            "name": "Which U.S. state is nicknamed The Natural State?",
            "answers": [
                "Alabama",
                "Alaska",
                "Arizona",
                "Arkansas"
            ],
            "correct": "Arkansas"
        },
        {
            "name": "What nickname has been given to the U.S. state of Oregon?",
            "answers": [
                "All of these",
                "Marmot State",
                "Beaver State",
                "Otter State"
            ],
            "correct": "Beaver State"
        },
        {
            "name": "What U.S. state is nicknamed Land of Infinite Variety?",
            "answers": [
                "South Carolina",
                "Florida",
                "Delaware",
                "South Dakota"
            ],
            "correct": "South Dakota"
        },
        {
            "name": "Which of these nicknames has been given to the U.S. state of Colorado?",
            "answers": [
                "Sweden of America",
                "Himalayas of America",
                "All of these",
                "Switzerland of America"
            ],
            "correct": "Switzerland of America"
        },
        {
            "name": "Which of these nicknames has been given to the U.S. state of Illinois?",
            "answers": [
                "Corn State",
                "Rubber State",
                "Cotton State",
                "Paper State"
            ],
            "correct": "Corn State"
        },
        {
            "name": "This U.S. state was nicknamed Vacationland, and the nickname is used on license plates.",
            "answers": [
                "Hawaii",
                "Maine",
                "Nevada",
                "California"
            ],
            "correct": "Maine"
        },
        {
            "name": "The Independence State is one of the nicknames of this U.S. state.",
            "answers": [
                "Virginia",
                "New Hampshire",
                "Pennsylvania",
                "West Virginia"
            ],
            "correct": "Pennsylvania"
        },
        {
            "name": "Which U.S. state is nicknamed The Cowboy State?",
            "answers": [
                "Texas",
                "Iowa",
                "Wyoming",
                "Idaho"
            ],
            "correct": "Wyoming"
        },
        {
            "name": "What nickname has been given to the U.S. state of Rhode Island?",
            "answers": [
                "Ocean State",
                "Marine State",
                "Island State",
                "All of these"
            ],
            "correct": "Ocean State"
        },
        {
            "name": "The nickname Queen State has been given to this U.S. state.",
            "answers": [
                "West Virginia",
                "Virginia",
                "All of these",
                "Maryland"
            ],
            "correct": "Maryland"
        },
        {
            "name": "Which country, Slovakia or Slovenia, used to be part of Yugoslavia?",
            "answers": [
                "Slovenia",
                "Both",
                "Slovakia",
                "None of them"
            ],
            "correct": "Slovenia"
        },
        {
            "name": "Which former empire were both Slovenia and Slovakia part of?",
            "answers": [
                "Austria-Hungary",
                "Byzantium",
                "Ottoman Empire",
                "Roman Empire"
            ],
            "correct": "Austria-Hungary"
        },
        {
            "name": "Slovakia and Slovenia joined the European Union on the same day (May 1, 2004). Which of them was first to accept the euro currency?",
            "answers": [
                "None of them has accepted the EURO currency.",
                "Both did on the day of their accession to the EU.",
                "Slovakia",
                "Slovenia"
            ],
            "correct": "Slovenia"
        },
        {
            "name": "What is the capital of Slovakia?",
            "answers": [
                "Ljubljana",
                "Prague",
                "Zagreb",
                "Bratislava"
            ],
            "correct": "Bratislava"
        },
        {
            "name": "What is the capital of Slovenia?",
            "answers": [
                "Prague",
                "Bratislava",
                "Ljubljana",
                "Zagreb"
            ],
            "correct": "Ljubljana"
        },
        {
            "name": "Which country borders both Slovenia and Slovakia?",
            "answers": [
                "Serbia",
                "Austria",
                "Poland",
                "Germany"
            ],
            "correct": "Austria"
        },
        {
            "name": "Which statement concerning Slovakia and Slovenia is true?",
            "answers": [
                "Both countries have access to the Black Sea.",
                "Slovenia has access to the Adriatic Sea.",
                "Slovakia has access to the Baltic Sea.",
                "Both countries are landlocked."
            ],
            "correct": "Slovenia has access to the Adriatic Sea."
        },
        {
            "name": "The Danube river flows through many Central European countries. Which of these statements concerning Slovakia and Slovenia is true?",
            "answers": [
                "The Danube does not flow through either of the 2 countries.",
                "The Danube originates in Slovakia and then enters Slovenia.",
                "Of the two countries, only Slovakia has Danubian banks.",
                "The Danube originates in Slovenia and then flows through Slovakia."
            ],
            "correct": "Of the two countries, only Slovakia has Danubian banks."
        },
        {
            "name": "Which of these statements regarding Slovakia and Slovenia is true?",
            "answers": [
                "Only Slovakia is an Alpine country.",
                "Only Slovenia is an Alpine country.",
                "None of them is an Alpine country.",
                "Both are Alpine countries."
            ],
            "correct": "Only Slovenia is an Alpine country."
        },
        {
            "name": "Which country, Slovakia or Slovenia, borders a country that did not join the European Union in the period 1951-2007?",
            "answers": [
                "Only Slovakia borders a country that did not join the EU during this period.",
                "Both share borders only with countries that became EU members during this period.",
                "Both share borders with countries that did not became EU members during this period.",
                "Only Slovenia borders a country that did not join the EU during this period."
            ],
            "correct": "Both share borders with countries that did not became EU members during this period."
        },
        {
            "name": "What design is depicted on the national flag of Switzerland?",
            "answers": [
                "Sun",
                "Cross",
                "Star",
                "Leaf"
            ],
            "correct": "Cross"
        },
        {
            "name": "Switzerland is often associated with this product which is produced at the highest quality in the country.",
            "answers": [
                "Chocolate",
                "Watches",
                "All of these",
                "Cheese"
            ],
            "correct": "All of these"
        },
        {
            "name": "What is the unofficial national motto of Switzerland?",
            "answers": [
                "Progress ans prosperity",
                "In God we trust",
                "One for all, all for one",
                "Higher and stronger"
            ],
            "correct": "One for all, all for one"
        },
        {
            "name": "What is the capital city of Switzerland?",
            "answers": [
                "Bonn",
                "Bern",
                "Berlin",
                "Zurich"
            ],
            "correct": "Bern"
        },
        {
            "name": "What mountain covers about 65% of the territory of Switzerland?",
            "answers": [
                "the Apennines",
                "the Andes",
                "Ararat",
                "the Alps"
            ],
            "correct": "the Alps"
        },
        {
            "name": "What language is adopted as official for the territory of Switzerland?",
            "answers": [
                "German",
                "Italian",
                "French",
                "All of these"
            ],
            "correct": "All of these"
        },
        {
            "name": "What is the title of the national anthem of Switzerland?",
            "answers": [
                "Swiss Vow",
                "Swiss Song",
                "Swiss Anthem",
                "Swiss Psalm"
            ],
            "correct": "Swiss Psalm"
        },
        {
            "name": "What is the main ingredient of the traditional Swiss dish Rosti?",
            "answers": [
                "Cheese",
                "Cabbage",
                "Fish",
                "Potatoes"
            ],
            "correct": "Potatoes"
        },
        {
            "name": "What is the largest city in Switzerland?",
            "answers": [
                "Geneva",
                "Zurich",
                "Bern",
                "Munich"
            ],
            "correct": "Zurich"
        },
        {
            "name": "Which of these famous philosophers was of Swiss descent?",
            "answers": [
                "Jean-Jacques Rousseau",
                "Michel Foucault",
                "Friedrich Schiller",
                "Jean-Paul Sartre"
            ],
            "correct": "Jean-Jacques Rousseau"
        },
        {
            "name": "Which of these is the capital and the largest city in North Korea?",
            "answers": [
                "Nampho",
                "Hungnam",
                "Sinuiju",
                "Pyongyang"
            ],
            "correct": "Pyongyang"
        },
        {
            "name": "What type of climate does North Korea have?",
            "answers": [
                "Oceanic",
                "Humid subtropical",
                "Continental",
                "Tropical"
            ],
            "correct": "Continental"
        },
        {
            "name": "Which of these seas surrounds the Korean Peninsula, in the northern part of which North Korea is located?",
            "answers": [
                "All of these",
                "East China Sea",
                "Sea of Japan",
                "Yellow Sea"
            ],
            "correct": "All of these"
        },
        {
            "name": "The establishment of the Democratic Peoples Republic of Korea in this year set the official beginning of North Koreas history.",
            "answers": [
                "1940",
                "1950",
                "1948",
                "1945"
            ],
            "correct": "1948"
        },
        {
            "name": "Which of these statements is true about the division of Korea into two parts?",
            "answers": [
                "Korea was divided at the 38th parallel.",
                "North Korea was to be administered by the Soviet union.",
                "All of these",
                "It followed the end of World War II."
            ],
            "correct": "All of these"
        },
        {
            "name": "Which of these natural disasters is known to hit the lands of North Korea on the regular?",
            "answers": [
                "All of these",
                "Droughts",
                "Floods",
                "Typhoons"
            ],
            "correct": "All of these"
        },
        {
            "name": "What is the official language that is spoken in North Korea?",
            "answers": [
                "All of these",
                "Russian",
                "Korean",
                "Japanese"
            ],
            "correct": "Korean"
        },
        {
            "name": "Which of these religious movements has many adherents among the people of North Korea?",
            "answers": [
                "Korean shamanism",
                "Cheondoism",
                "All of these",
                "Buddhism"
            ],
            "correct": "All of these"
        },
        {
            "name": "Which of these musicians became the first US musical group ever to perform in North Korea?",
            "answers": [
                "The Black Eyed Peas",
                "The New York Philharmonic Orchestra",
                "The Helen Henderson Choir of Victory Baptist Church",
                "Linkin Park"
            ],
            "correct": "The New York Philharmonic Orchestra"
        },
        {
            "name": "Which of these events is regularly performed only in North Korea?",
            "answers": [
                "Mass games",
                "Senior citizens beauty pageant",
                "Politicians athletics",
                "Binoculars football games"
            ],
            "correct": "Mass games"
        },
        {
            "name": "What other name is commonly used to refer to the Colorado River in North America?",
            "answers": [
                "Black River",
                "Yellow River",
                "Blue River",
                "Red River"
            ],
            "correct": "Red River"
        },
        {
            "name": "What is the name of the longest river in the United States of America?",
            "answers": [
                "Colorado",
                "Missouri",
                "Rio Grande",
                "Mississippi"
            ],
            "correct": "Missouri"
        },
        {
            "name": "The Ohio River does not flow though the territory of this American state.",
            "answers": [
                "Illinois",
                "Virginia",
                "Kentucky",
                "West Virginia"
            ],
            "correct": "Virginia"
        },
        {
            "name": "What is the longest river located entirely within the territory of the state of California?",
            "answers": [
                "Mad River",
                "Salmon River",
                "Big River",
                "Sacramento River"
            ],
            "correct": "Sacramento River"
        },
        {
            "name": "What is the meaning of the name of the Mississippi River, which originated from the Ojibwe word misi-ziibi?",
            "answers": [
                "Holy water",
                "Fast river",
                "River of life",
                "Great river"
            ],
            "correct": "Great river"
        },
        {
            "name": "What ocean fish migrates from the Pacific Ocean to the Columbia River to spawn at the end of their life cycles?",
            "answers": [
                "Steelhead salmon (Rainbow trout)",
                "All of these",
                "Coho salmon",
                "Chinook salmon"
            ],
            "correct": "All of these"
        },
        {
            "name": "What is the The Rio Grande river called in Mexico?",
            "answers": [
                "All of these",
                "Rio Bravo",
                "Rio Rojo",
                "Rio del Fuego"
            ],
            "correct": "Rio Bravo"
        },
        {
            "name": "What river in North America provides food for the largest wintering Bald Eagle population in the continental United States?",
            "answers": [
                "Fraser",
                "Skeena",
                "St. Lawrence",
                "Skagit River"
            ],
            "correct": "Skagit River"
        },
        {
            "name": "What does the Mohawk name of the Saint Lawrence River, Kaniatarowanenneh, mean?",
            "answers": [
                "Female river",
                "Murmur of the gods",
                "Burning water",
                "Big waterway"
            ],
            "correct": "Big waterway"
        },
        {
            "name": "More than half of the Yukon River is located within this American state.",
            "answers": [
                "North Dakota",
                "Kansas",
                "Alaska",
                "Texas"
            ],
            "correct": "Alaska"
        },
        {
            "name": "What is the name of the capital and largest city of Norway?",
            "answers": [
                "Reykjavik",
                "Bergen",
                "Oslo",
                "Helsinki"
            ],
            "correct": "Oslo"
        },
        {
            "name": "According to some medieval texts, the name of Norway originated from the name of which of the following?",
            "answers": [
                "A mountain",
                "The name of a king",
                "The name of a river",
                "An animal"
            ],
            "correct": "The name of a king"
        },
        {
            "name": "What type of government does Norway have?",
            "answers": [
                "Absolute monarchy",
                "Constitutional monarchy",
                "Democratic republic",
                "Presidential republic"
            ],
            "correct": "Constitutional monarchy"
        },
        {
            "name": "What animal is depicted on the Coat of Arms of Norway?",
            "answers": [
                "Lion",
                "Bear",
                "Deer",
                "Wolf"
            ],
            "correct": "Lion"
        },
        {
            "name": "What is the predominant form of Christianity in Norway?",
            "answers": [
                "Anglicanism",
                "Eastern Orthodox Christianity",
                "Roman Catholicism",
                "Protestantism"
            ],
            "correct": "Protestantism"
        },
        {
            "name": "What is the official language adopted by the government of Norway?",
            "answers": [
                "Norwegian and Finnish",
                "Norwegian and English",
                "Norwegian and Romani",
                "Norwegian"
            ],
            "correct": "Norwegian"
        },
        {
            "name": "What is the title of the national anthem of Norway?",
            "answers": [
                "Glory, Glory",
                "Oh, Amazing Land",
                "The Way North",
                "Yes, We Love This Country"
            ],
            "correct": "Yes, We Love This Country"
        },
        {
            "name": "What is Bunad?",
            "answers": [
                "A traditional Norwegian celebration",
                "A Norwegian song",
                "A Norwegian costume",
                "A Norwegian meal"
            ],
            "correct": "A Norwegian costume"
        },
        {
            "name": "What is the popular Norwegian delicacy called Smalahove made from?",
            "answers": [
                "Pork intestines",
                "Fish liver",
                "Chicken popes nose",
                "Lamb head"
            ],
            "correct": "Lamb head"
        },
        {
            "name": "Which of the following is present on the national flag of Norway?",
            "answers": [
                "Star",
                "Cross",
                "Sun",
                "Mountain"
            ],
            "correct": "Cross"
        },
        {
            "name": "What is the largest carnivorous mammal endemic to the island of Madagascar?",
            "answers": [
                "Short-tailed Mongoose",
                "Fanaloka",
                "Meerkat",
                "Fossa"
            ],
            "correct": "Fossa"
        },
        {
            "name": "Which of these statements is true about Madagascar?",
            "answers": [
                "It is slightly smaller than France.",
                "It is the worlds 56th-largest country in terms of area.",
                "None of these",
                "It is the fifth largest island in the world."
            ],
            "correct": "None of these"
        },
        {
            "name": "What is the title of the national anthem of Madagascar, written by Norbert Raharisoa and composed by Rahajason?",
            "answers": [
                "Dear Motherland",
                "Blessed Island",
                "Oh, Our Beloved Fatherland",
                "Rise, Oh, Rise"
            ],
            "correct": "Oh, Our Beloved Fatherland"
        },
        {
            "name": "What are the colors on the national flag of Madagascar, adopted on October 14, 1958?",
            "answers": [
                "Red, yellow and blue",
                "Yellow and green",
                "Blue, green and yellow",
                "White, red and green"
            ],
            "correct": "White, red and green"
        },
        {
            "name": "What city is the capital and largest city of Madagascar?",
            "answers": [
                "Fianarantsoa",
                "Mahajanga",
                "Antananarivo",
                "Toliara"
            ],
            "correct": "Antananarivo"
        },
        {
            "name": "What are the official languages spoken in Madagascar?",
            "answers": [
                "Malagasy, French and English",
                "Malagasy and English",
                "Malagasy and French",
                "English and French"
            ],
            "correct": "Malagasy, French and English"
        },
        {
            "name": "What is the name some ecologists use wen referring to Madagascar?",
            "answers": [
                "The eight continent",
                "All of these",
                "The island of Adam and Eve",
                "The garden of Eden"
            ],
            "correct": "The eight continent"
        },
        {
            "name": "What kind of traditional ritual is the Famadihana, practiced by the Malagasy people in Madagascar?",
            "answers": [
                "Circumcision",
                "Introducing boys into manhood by tattooing and scarring them",
                "Turning of the dead",
                "Animal sacrifice"
            ],
            "correct": "Turning of the dead"
        },
        {
            "name": "What type of meal is the godro-godro, which is very popular in the Madagascar cuisine?",
            "answers": [
                "Seafood meal",
                "Rice meal",
                "Pudding",
                "Vegetable stew"
            ],
            "correct": "Pudding"
        },
        {
            "name": "This tree is a very important part of the Madagascar dry deciduous forests and six species of it are endemic to the island.",
            "answers": [
                "All of these",
                "Bottle tree",
                "Monkey bread tree",
                "Upside-down tree"
            ],
            "correct": "All of these"
        },
        {
            "name": "In 1945 Borneo was the site of horrific acts of brutality. The Japanese forced over 6,000 POWs and island civilians to march to a new camp some 160 miles away. This became known as the Sandakan Death March, as only 200 marchers survived to the end of the war.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "How many new species of animals and plants were discovered on Borneo between July 2005 and September 2006?",
            "answers": [
                "None",
                "121",
                "3",
                "52"
            ],
            "correct": "52"
        },
        {
            "name": "Between the 1980s and 2006 Borneo lost approximately 45 % of its rain forests due to logging.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Borneo is the 3rd largest island in the world.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "From the 1600s to the 1800s, the British and Dutch were interested in Borneo because they wanted control of this resource.",
            "answers": [
                "Pepper",
                "Salt",
                "Coffee",
                "Tea"
            ],
            "correct": "Pepper"
        },
        {
            "name": "Which of the characteristics below is typical for the buildings on the Greek archipelago of Santorini?",
            "answers": [
                "The white-and-blue colouring",
                "The huge balconies",
                "The red-tiled roofs",
                "The arched doors"
            ],
            "correct": "The white-and-blue colouring"
        },
        {
            "name": "What is the biggest Greek island ?",
            "answers": [
                "Rhodes",
                "Cyprus",
                "Euboea",
                "Crete"
            ],
            "correct": "Crete"
        },
        {
            "name": "The Navagio Bay (also known as the Shipwreck) is the most popular beach of which Greek island?",
            "answers": [
                "Mykonos",
                "Zakynthos",
                "Lefkada",
                "Thasos"
            ],
            "correct": "Zakynthos"
        },
        {
            "name": "Halkidiki, one of the Greek peninsulas, has a peculiar form which resembles what?",
            "answers": [
                "A horseshoe",
                "A trident",
                "An olive twig",
                "The Greek letter Omega"
            ],
            "correct": "A trident"
        },
        {
            "name": "Which medieval monument is an essential part of the archaeological sites of the Greek island Rhodes?",
            "answers": [
                "The Palace of the Grand Master",
                "The White Tower",
                "The Venice Castle of Naoussa",
                "The Odeon"
            ],
            "correct": "The Palace of the Grand Master"
        },
        {
            "name": "Which island is NOT a part of Greece?",
            "answers": [
                "Kea",
                "Hydra",
                "Gozo",
                "Kos"
            ],
            "correct": "Gozo"
        },
        {
            "name": "The Corinth Canal separates this Greek peninsula from the mainland of Greece.",
            "answers": [
                "Halkidiki",
                "Mani",
                "Elounda",
                "Peloponnese"
            ],
            "correct": "Peloponnese"
        },
        {
            "name": "Which is the largest group of Greek islands?",
            "answers": [
                "The Cyclades",
                "The Sporades",
                "The Ionian",
                "The East Aegean"
            ],
            "correct": "The Cyclades"
        },
        {
            "name": "The Cave of Melissani is among the most popular geological landmarks of which Greek island?",
            "answers": [
                "Kefallonia",
                "Lefkada",
                "Mykonos",
                "Samos"
            ],
            "correct": "Kefallonia"
        },
        {
            "name": "Which of the following places in Greece is NOT an island?",
            "answers": [
                "Alonissos",
                "Ithaka",
                "Kavala",
                "Kythira"
            ],
            "correct": "Kavala"
        },
        {
            "name": "The Shinsengumi group remained loyal to the Tokugawa shogunate until the end of the Bakamatsu.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What is the name of the leader of the first squad of Shinsengumi?",
            "answers": [
                "Okita Sōji",
                "Inoue Genzaburō",
                "Harada Sanosuke",
                "Tōdō Heisuke"
            ],
            "correct": "Okita Sōji"
        },
        {
            "name": "At its peak, how many members did the Shinsengumi group have?",
            "answers": [
                "50+",
                "300+",
                "200+",
                "100+"
            ],
            "correct": "300+"
        },
        {
            "name": "According to the Shinsengumi rules, if the leader of a unit is mortally wounded in a fight, what should the rest of the group do?",
            "answers": [
                "Run away",
                "Take the body and run away",
                "Commit Seppuku",
                "All members of the unit must fight and die on the spot."
            ],
            "correct": "All members of the unit must fight and die on the spot."
        },
        {
            "name": "In which city was the Shinsengumi group founded?",
            "answers": [
                "Kyoto",
                "Tokyo",
                "Nagasaki",
                "Osaka"
            ],
            "correct": "Kyoto"
        },
        {
            "name": "Who was Shinsengumi Commander from 1863 to 1868?",
            "answers": [
                "Nagakura Shinpachi",
                "Harada Sanosuke",
                "Tōdō Heisuke",
                "Kondō Isami"
            ],
            "correct": "Kondō Isami"
        },
        {
            "name": "The code of Shinsengumi comprised five rules. What was the punishment for breaking these rules?",
            "answers": [
                "Sepuku",
                "Cutting of one finger",
                "Exile",
                "Paying six gold coins"
            ],
            "correct": "Sepuku"
        },
        {
            "name": "What was the favorite weapon of Harada Sanosuke, the 10th unit captain of the Shinsengumi?",
            "answers": [
                "Kozuka Blade",
                "Spear",
                "Sword",
                "Shuriken"
            ],
            "correct": "Spear"
        },
        {
            "name": "This river flows through the southern part of England and passes through the capital of the country.",
            "answers": [
                "The Avon",
                "All of these",
                "The Thames",
                "The Mersey"
            ],
            "correct": "The Thames"
        },
        {
            "name": "What river is considered the longest river on the planet?",
            "answers": [
                "The Nile",
                "The Tigris",
                "The Mississippi",
                "The Amazon"
            ],
            "correct": "The Nile"
        },
        {
            "name": "What is the name of the holy river of the Hindu people?",
            "answers": [
                "The Ganges",
                "The Euphrates",
                "The Indus",
                "The Tigris"
            ],
            "correct": "The Ganges"
        },
        {
            "name": "What river flows through the central part of the French capital, Paris?",
            "answers": [
                "Seine",
                "Loire",
                "Charente",
                "Rhine"
            ],
            "correct": "Seine"
        },
        {
            "name": "What is the name of the longest river in Asia, and the third-longest in the world?",
            "answers": [
                "The Yangtze",
                "The Mekong-Lancang",
                "The Tigris",
                "The Huang He"
            ],
            "correct": "The Yangtze"
        },
        {
            "name": "This river is the longest in the United States, and it gave its name to one of the states.",
            "answers": [
                "The Missouri River",
                "The Mississippi River",
                "The Alabama River",
                "The Ohio River"
            ],
            "correct": "The Missouri River"
        },
        {
            "name": "What is the largest river on the planet by volume?",
            "answers": [
                "The Mississippi",
                "The Amazon",
                "The Nile",
                "The Ganges"
            ],
            "correct": "The Amazon"
        },
        {
            "name": "What is the name of the river in which Jesus Christ was baptized by John the Baptist?",
            "answers": [
                "The Tigris",
                "The Jordan",
                "The Euphrates",
                "The Nile"
            ],
            "correct": "The Jordan"
        },
        {
            "name": "This North American river is also called Kaniatarowanenneh, which means big waterway in Mohawk.",
            "answers": [
                "Saint Lawrence River",
                "Rio Grande",
                "Colorado",
                "Ohio River"
            ],
            "correct": "Saint Lawrence River"
        },
        {
            "name": "This European river originates in Germany and empties in the Black Sea via a delta.",
            "answers": [
                "The Volga",
                "The Danube",
                "The Rhine",
                "All of these"
            ],
            "correct": "The Danube"
        },
        {
            "name": "There is a city in Oklahoma called Moon.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Oklahoma has no cities and communities named after Presidents.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "In what Oklahoma city was the shopping cart invented?",
            "answers": [
                "Ardmore",
                "Guthrie",
                "Duncan",
                "Broken Arrow"
            ],
            "correct": "Ardmore"
        },
        {
            "name": "How many of the astronauts who have flown in space between 1959 and 2009 were from Oklahoma?",
            "answers": [
                "6",
                "20",
                "0",
                "3"
            ],
            "correct": "6"
        },
        {
            "name": "There is a town in Oklahoma called IXL.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "A windspeed of 100 feet above the ground was observed in 1999 in what Oklahoma city?",
            "answers": [
                "Lawton",
                "Norman",
                "Moore",
                "Midwest City"
            ],
            "correct": "Moore"
        },
        {
            "name": "Will Rogers World Airport and the Wiley Post Airport were named after people who died in 1935. How did they meet their demise?",
            "answers": [
                "in a plane crash",
                "by murder",
                "in a car crash",
                "as a result of a boat accident"
            ],
            "correct": "in a plane crash"
        },
        {
            "name": "Carl Albert High School was named after Carl Bert Albert, who was the Speaker of the House of Representatives from 1971 to 1977. What small Oklahoma town did he hail from?",
            "answers": [
                "Enid",
                "Bushyhead",
                "Slaughterville",
                "Bugtussle"
            ],
            "correct": "Bugtussle"
        },
        {
            "name": "Oklahoma has more man-made lakes than any other state.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Which of the following is NOT an Oklahoma city?",
            "answers": [
                "Peoria",
                "Burbank",
                "St. Louis",
                "Toledo"
            ],
            "correct": "Toledo"
        },
        {
            "name": "Boise City, Oklahoma was the only city within the continental United States to be bombed during World War II.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The national motto of the Republic of Botswana is Pula. What does this mean in English?",
            "answers": [
                "Rain",
                "Strength",
                "Wisdom",
                "Sunshine"
            ],
            "correct": "Rain"
        },
        {
            "name": "The national motto of what country is A Mari Usque Ad Mare, meaning From Sea to Sea in Latin?",
            "answers": [
                "Jamaica",
                "Spain",
                "Canada",
                "Haiti"
            ],
            "correct": "Canada"
        },
        {
            "name": "What does the national motto of India, Satyameva Jayate, mean in Sanskrit?",
            "answers": [
                "The Supreme Treasure of Truth",
                "Truth Alone Triumphs",
                "Truth Is The Best Way",
                "Truth Is Divine"
            ],
            "correct": "Truth Alone Triumphs"
        },
        {
            "name": "The national motto of what country is Peace at Home, Peace in the World?",
            "answers": [
                "Afghanistan",
                "Somalia",
                "Nigeria",
                "Turkey"
            ],
            "correct": "Turkey"
        },
        {
            "name": "The national motto of this South American country is Libertad o muerte, Spanish for Liberty or Death.",
            "answers": [
                "Peru",
                "Uruguay",
                "Argentina",
                "Brazil"
            ],
            "correct": "Uruguay"
        },
        {
            "name": "The national motto of the Republic of the Fiji Islands is Rerevaka na Kalou ka Doka na Tui. What does this mean?",
            "answers": [
                "Fear God and honor the Queen",
                "God bless our beautiful motherland",
                "We kneel before you, blessed land",
                "Hard work comes before prosperity"
            ],
            "correct": "Fear God and honor the Queen"
        },
        {
            "name": "The slogan Sub Umbra Floreo, or Under the Shade I Flourish is the national motto of what Central American country?",
            "answers": [
                "Costa Rica",
                "Belize",
                "Guatemala",
                "Nicaragua"
            ],
            "correct": "Belize"
        },
        {
            "name": "The national motto of what island country is Ever Conscious of God We Aspire, Build and Advance as One People?",
            "answers": [
                "Grenada",
                "New Zealand",
                "Papua New Guinea",
                "Micronesia"
            ],
            "correct": "Grenada"
        },
        {
            "name": "The national motto of this Arabic country is God is the greatest.",
            "answers": [
                "Morocco",
                "Iran",
                "Syria",
                "Iraq"
            ],
            "correct": "Iraq"
        },
        {
            "name": "What is the meaning of the national motto of the Republic of Indonesia, Bhinneka Tunggal Ika?",
            "answers": [
                "Unity in Diversity",
                "Past Present Future",
                "Progress and Stability",
                "We Are Strong Together"
            ],
            "correct": "Unity in Diversity"
        },
        {
            "name": "In what country is the Angel Falls, the highest waterfall on the planet, located?",
            "answers": [
                "Brazil",
                "Argentina",
                "Colombia",
                "Venezuela"
            ],
            "correct": "Venezuela"
        },
        {
            "name": "Which of these is a section of the famous Niagara Falls?",
            "answers": [
                "Horseback Falls",
                "Horseshoe Falls",
                "Horsetail Falls",
                "Horseman Falls"
            ],
            "correct": "Horseshoe Falls"
        },
        {
            "name": "In what U.S. state is the famous Bridalveil Fall located?",
            "answers": [
                "California",
                "Florida",
                "Colorado",
                "Hawaii"
            ],
            "correct": "California"
        },
        {
            "name": "What is the name of the tallest waterfall made by humans, which is located in Umbria, Italy?",
            "answers": [
                "Cascata delle Marmore",
                "Cascata del Maria",
                "Cascata del Mare",
                "Cascata delle Maccheroni"
            ],
            "correct": "Cascata delle Marmore"
        },
        {
            "name": "Jog Falls is the highest plunge waterfall in this Asian country.",
            "answers": [
                "India",
                "Japan",
                "China",
                "Thailand"
            ],
            "correct": "India"
        },
        {
            "name": "What is the name of the second-tallest waterfall on the planet?",
            "answers": [
                "Temola Falls",
                "Tugela Falls",
                "Tierra Falls",
                "Tamilla Falls"
            ],
            "correct": "Tugela Falls"
        },
        {
            "name": "In what European country are the Rhine Falls located?",
            "answers": [
                "Germany",
                "Switzerland",
                "Austria",
                "Belgium"
            ],
            "correct": "Switzerland"
        },
        {
            "name": "In what U.S. state is the tallest waterfall in the country, Waihilau Falls, located?",
            "answers": [
                "Idaho",
                "Wyoming",
                "Hawaii",
                "Iowa"
            ],
            "correct": "Hawaii"
        },
        {
            "name": "In what country is Jurong Falls, the tallest artificial waterfall in the world, located?",
            "answers": [
                "Indonesia",
                "Singapore",
                "Vietnam",
                "North Korea"
            ],
            "correct": "Singapore"
        },
        {
            "name": "What is the name of the tallest waterfall in eastern North America?",
            "answers": [
                "Pissing Mare Falls",
                "Pissing Horse Falls",
                "Pissing Elephant Falls",
                "Pissing Buffalo Falls"
            ],
            "correct": "Pissing Mare Falls"
        },
        {
            "name": "What is the old name of the country of Madagascar?",
            "answers": [
                "Rhodesia",
                "Mozambique",
                "Malaysia",
                "Malagasy Republic"
            ],
            "correct": "Malagasy Republic"
        },
        {
            "name": "Where is Madagascar?",
            "answers": [
                "Off the Southeast Coast of South America",
                "Off the Southeast Coast of South America",
                "Off the Southwest Coast of Africa",
                "Off the Southeast Coast of Africa"
            ],
            "correct": "Off the Southeast Coast of Africa"
        },
        {
            "name": "What are the 2 main languages spoken in Madagascar?",
            "answers": [
                "Italian and French",
                "French and English",
                "English and Malagasy",
                "French and Malagasy"
            ],
            "correct": "French and Malagasy"
        },
        {
            "name": "Madagascar is the worlds largest producer and exporter of what product?",
            "answers": [
                "Sugarcane",
                "Vanilla",
                "Tapioca",
                "Rice"
            ],
            "correct": "Vanilla"
        },
        {
            "name": "Of the 10,000 plants native to Madagascar, what percent are found nowhere else in the world?",
            "answers": [
                "49%",
                "13%",
                "90%",
                "72%"
            ],
            "correct": "90%"
        },
        {
            "name": "On what island is the country of Madagascar located?",
            "answers": [
                "Madagascar Island",
                "Nias Island",
                "Simeulue Island",
                "Christmas Island"
            ],
            "correct": "Madagascar Island"
        },
        {
            "name": "Which of the following is NOT located in the country of Madagascar?",
            "answers": [
                "Nosy-Be",
                "Antananarivo",
                "Antsirabe",
                "Bodrum"
            ],
            "correct": "Bodrum"
        },
        {
            "name": "I am a primate endemic to Madagascar. I have large, reflective eyes. I have a long tail. I have long toes, and opposable thumbs, useful for grasping tree branches. What am I?",
            "answers": [
                "An iguana",
                "An ape",
                "A monkey",
                "A lemur"
            ],
            "correct": "A lemur"
        },
        {
            "name": "What Portuguese explorer was the first to sight the island of Madagascar?",
            "answers": [
                "Henry the Navigator",
                "Diego Dias",
                "Diogo Cao",
                "Vasco da Gama"
            ],
            "correct": "Diego Dias"
        },
        {
            "name": "The first people to settle Madagascar came from Africa.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Where is New Guinea ranked in terms of size, among the islands of the world?",
            "answers": [
                "New Guinea is the third largest island in the world.",
                "New Guinea is the fourth largest island in the world.",
                "New Guinea is the second largest island in the world.",
                "New Guinea is the largest island in the world."
            ],
            "correct": "New Guinea is the second largest island in the world."
        },
        {
            "name": "New Guinea is an island north of Australia. It is divided into 2 roughly equal halves. What are they?",
            "answers": [
                "The western portion is part of Australia. The eastern portion is the country called Sumatra.",
                "The western portion is part of Indonesia. The eastern portion is the country called Papua New Guinea.",
                "The western portion is part of Indonesia. The eastern portion is the country called Sumatra.",
                "The western portion is part of Australia. The eastern portion is the country called Papua New Guinea."
            ],
            "correct": "The western portion is part of Indonesia. The eastern portion is the country called Papua New Guinea."
        },
        {
            "name": "New Guinea is the most linguistically diverse area in the world. Approximately how many separate languages are spoken there?",
            "answers": [
                "80",
                "20",
                "650",
                "1000"
            ],
            "correct": "1000"
        },
        {
            "name": "The island of New Guinea was named after the country of Guinea. Which is Guinea located?",
            "answers": [
                "Africa",
                "In the Pacific Ocean",
                "South America",
                "Asia"
            ],
            "correct": "Africa"
        },
        {
            "name": "The country of Papua New Guinea gained its independence from this country in 1975.",
            "answers": [
                "Mozambique",
                "Australia",
                "Indonesia",
                "New Zealand"
            ],
            "correct": "Australia"
        },
        {
            "name": "The west half of the island of New Guinea is part of Indonesia, which comprises how many islands?",
            "answers": [
                "4",
                "8,407",
                "17,508",
                "247,542"
            ],
            "correct": "17,508"
        },
        {
            "name": "Approximately what percentage of the population of the country of Papua New Guinea lives in rural areas?",
            "answers": [
                "50%",
                "34%",
                "97%",
                "86%"
            ],
            "correct": "86%"
        },
        {
            "name": "In the country of Papua New Guinea, how many children does the average woman give birth to during her lifetime according to 2006 estimates?",
            "answers": [
                "1.2",
                "2.8",
                "4.6",
                "3.9"
            ],
            "correct": "4.6"
        },
        {
            "name": "What is the capital city of the country of Papua New Guinea?",
            "answers": [
                "Port Moresby",
                "Madang",
                "Wau",
                "Lae"
            ],
            "correct": "Port Moresby"
        },
        {
            "name": "The islands of the Pacific are collectively termed Oceania. Oceania is divided into several sub-groupings. In which of these is New Guinea located?",
            "answers": [
                "Polynesia",
                "None of these",
                "Melanesia",
                "Micronesia"
            ],
            "correct": "Melanesia"
        },
        {
            "name": "What is the literal translation of the title of the national anthem of Egypt, Bilady, Bilady, Bilady?",
            "answers": [
                "Amazing, amazing, amazing",
                "Sing, sing, sing",
                "My country, my country, my country",
                "Blessed, blessed, blessed"
            ],
            "correct": "My country, my country, my country"
        },
        {
            "name": "What sea borders Egypt to the east?",
            "answers": [
                "Black Sea",
                "Yellow Sea",
                "Mediterranean Sea",
                "Red Sea"
            ],
            "correct": "Red Sea"
        },
        {
            "name": "Why was Egypt called Kemet, or black land, back in the ancient times?",
            "answers": [
                "Because of the coal",
                "Because of the black soils",
                "Because of the petrol",
                "All of these"
            ],
            "correct": "Because of the black soils"
        },
        {
            "name": "What is the official language spoken in Egypt?",
            "answers": [
                "Arabic and French",
                "Arabic",
                "Arabic, English and French",
                "Arabic and English"
            ],
            "correct": "Arabic"
        },
        {
            "name": "What Egyptian city is the largest city not only in Africa, but also in the whole Middle East?",
            "answers": [
                "Cairo",
                "Alexandria",
                "Giza",
                "Ismailia"
            ],
            "correct": "Cairo"
        },
        {
            "name": "What animal is depicted on Egypts national emblem, which is featured on the national flag of Egypt, adopted in 1984?",
            "answers": [
                "Bear",
                "Jackal",
                "Eagle",
                "Lion"
            ],
            "correct": "Eagle"
        },
        {
            "name": "Where was Omar Sharif, one of the most famous Egyptian actors, born?",
            "answers": [
                "Paris, France",
                "New York City, USA",
                "Hamburg, Germany",
                "Alexandria, Egypt"
            ],
            "correct": "Alexandria, Egypt"
        },
        {
            "name": "What sport is rated as the most popular sport among Egyptians?",
            "answers": [
                "Football (soccer)",
                "Baseball",
                "Handball",
                "Tennis"
            ],
            "correct": "Football (soccer)"
        },
        {
            "name": "The Egyptian territory is calculated to be roughly twice as large as the territory of this European country.",
            "answers": [
                "Poland",
                "Spain",
                "France",
                "United Kingdom"
            ],
            "correct": "Spain"
        },
        {
            "name": "What expression is commonly used in Egypt to refer to the annual flooding of the river Nile, the longest river in the world?",
            "answers": [
                "Feeding of the Soil",
                "The Gift of the Nile",
                "The Blessing of the Nile",
                "The Mother of Abundance"
            ],
            "correct": "The Gift of the Nile"
        },
        {
            "name": "Luxembourg is bordered by all of the following nations except this one.",
            "answers": [
                "Belgium",
                "France",
                "Germany",
                "Netherlands"
            ],
            "correct": "Netherlands"
        },
        {
            "name": "What languages are spoken in Luxembourg?",
            "answers": [
                "All of these",
                "German",
                "French",
                "Luxembourgish"
            ],
            "correct": "All of these"
        },
        {
            "name": "Luxembourg was not involved in any 20th century war.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Which of the following organizations is Luxembourg a member of?",
            "answers": [
                "OECD",
                "United Nations",
                "European Union",
                "All of these"
            ],
            "correct": "All of these"
        },
        {
            "name": "Luxembourg has a population of approximately how many people?",
            "answers": [
                "30,000",
                "one half million",
                "two million",
                "three and one half million"
            ],
            "correct": "one half million"
        },
        {
            "name": "Why is Luxembourg City one of the wealthiest cities in the world?",
            "answers": [
                "It is the European center of computer and computer related companies.",
                "There is great mineral wealth in the area around the city.",
                "It is a banking and administrative center.",
                "Rich retired people have ranked it as the most desirable place in Europe to live."
            ],
            "correct": "It is a banking and administrative center."
        },
        {
            "name": "What is the official name of Luxembourg?",
            "answers": [
                "The Peoples Republic of Luxembourg",
                "Grand Duchy of Luxembourg",
                "The Luxembourg Democratic Republic",
                "The Kingdom of Luxembourg"
            ],
            "correct": "Grand Duchy of Luxembourg"
        },
        {
            "name": "How does Luxembourg, which is a member of NATO, contribute to NATO?",
            "answers": [
                "by maintaining the sixth largest NATO air force",
                "by maintaining a small army",
                "by allowing extensive US and British military installations on its territory",
                "by financially supporting a navy even though is has no coastline"
            ],
            "correct": "by maintaining a small army"
        },
        {
            "name": "What is the predominant religion in Luxembourg?",
            "answers": [
                "Islam",
                "Orthodox Catholic",
                "Protestant",
                "Roman Catholic"
            ],
            "correct": "Roman Catholic"
        },
        {
            "name": "The flag of Luxembourg consists of three vertical stripes of red, blue, green.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "In what country is Virginia Falls located?",
            "answers": [
                "United States",
                "Austarlia",
                "Wales",
                "Canada"
            ],
            "correct": "Canada"
        },
        {
            "name": "What is the name of the tallest waterfall in Asia?",
            "answers": [
                "Midagahara Falls",
                "Hannoki Falls",
                "Shomyo Falls",
                "Toyama Falls"
            ],
            "correct": "Hannoki Falls"
        },
        {
            "name": "The Eas a Chual Aluinn is the highest waterfall in this country.",
            "answers": [
                "France",
                "China",
                "Sri Lanka",
                "Great Britain"
            ],
            "correct": "Great Britain"
        },
        {
            "name": "What is the English name of the Mosi-oa-Tunya waterfall located between Zambia and Zimbabwe?",
            "answers": [
                "Virginia Falls",
                "Veronica Falls",
                "Valeria Falls",
                "Victoria Falls"
            ],
            "correct": "Victoria Falls"
        },
        {
            "name": "In what country is Yumbilla Falls, the fifth-tallest waterfall on the planet, located?",
            "answers": [
                "Chile",
                "Peru",
                "Thailand",
                "Australia"
            ],
            "correct": "Peru"
        },
        {
            "name": "Where is the third-tallest waterfall on the planet, Ramnefjellsfossen, located?",
            "answers": [
                "Norway",
                "Finland",
                "Switzerland",
                "Denmark"
            ],
            "correct": "Norway"
        },
        {
            "name": "What does the name of the Iguazu Falls, located in South America, mean in English?",
            "answers": [
                "Way to heaven",
                "River storm",
                "Devils waterfall",
                "Big water"
            ],
            "correct": "Big water"
        },
        {
            "name": "In what country is Gocta Waterfall located?",
            "answers": [
                "Venezuela",
                "Peru",
                "Spain",
                "Nicaragua"
            ],
            "correct": "Peru"
        },
        {
            "name": "What is the name of the highest waterfall in North America?",
            "answers": [
                "Richard Brown Falls",
                "Johnathan Stevens Falls",
                "James Bruce Falls",
                "John Cliff Falls"
            ],
            "correct": "James Bruce Falls"
        },
        {
            "name": "In which U.S. state is the Multnomah Falls located?",
            "answers": [
                "Ohio",
                "Oregon",
                "Maine",
                "Missouri"
            ],
            "correct": "Oregon"
        },
        {
            "name": "What is the name of the largest volcano on Earth in terms of area?",
            "answers": [
                "Mauna Loa",
                "Taal Volcano",
                "Mauna Kea",
                "Mount Etna"
            ],
            "correct": "Mauna Loa"
        },
        {
            "name": "In what Asian country is the Mount Unzen volcanic group located?",
            "answers": [
                "Vietnam",
                "Japan",
                "China",
                "Thailand"
            ],
            "correct": "Japan"
        },
        {
            "name": "The area around what Italian volcano is the most densely populated volcanic region in the world, making the volcano one of the most dangerous on the planet?",
            "answers": [
                "Mount Vesuvius",
                "Mount Etna",
                "None of these",
                "Stromboli"
            ],
            "correct": "Mount Vesuvius"
        },
        {
            "name": "This Andean stratovolcano in Colombia erupted in 1993, killing 9 people.",
            "answers": [
                "Galeras",
                "Pasto",
                "Nina",
                "Paloma"
            ],
            "correct": "Galeras"
        },
        {
            "name": "The Santorini volcanic islands are part of what European country?",
            "answers": [
                "Denmark",
                "Spain",
                "Italy",
                "Greece"
            ],
            "correct": "Greece"
        },
        {
            "name": "This Russian volcano, located on the Kamchatka Peninsula, is one of the most active volcanoes on the planet.",
            "answers": [
                "Elbrus",
                "Anik Mountain",
                "Sikhote-Alin",
                "Avachinsky"
            ],
            "correct": "Avachinsky"
        },
        {
            "name": "What is the name of the dangerous active volcano located in the Western Highlands of Guatemala?",
            "answers": [
                "Santa Lucia",
                "Santa Fernanda",
                "Santa Beatrice",
                "Santa Maria"
            ],
            "correct": "Santa Maria"
        },
        {
            "name": "On which of the Philippine islands is the active Taal Volcano located?",
            "answers": [
                "Mindanao",
                "Visayas",
                "Luzon",
                "None of these"
            ],
            "correct": "Luzon"
        },
        {
            "name": "On which of the Canary Islands is the Mount Teide volcano located?",
            "answers": [
                "Gran Canaria",
                "Fuerteventura",
                "La Palma",
                "Tenerife"
            ],
            "correct": "Tenerife"
        },
        {
            "name": "None of these  could be found in Europe before the Great Geographical Discoveries: Potatoes, chocolate, cocoa, quinine, tobacco, cabbage, cane-sugar, furs, whale oil, indigo, tea, coffee, porcelain, and cotton silks .",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "What is the highest point in the state of Illinois?",
            "answers": [
                "Lincoln Mound",
                "Devils Mound",
                "Charles Mound",
                "Mount Vernon"
            ],
            "correct": "Charles Mound"
        },
        {
            "name": "Makawao is the capital of the State of Hawaii.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "Who is the 16th president of the United States?",
            "answers": [
                "Abraham Lincoln",
                "Andrew Johnson",
                "James Polk",
                "Ulysses P. Grant"
            ],
            "correct": "Abraham Lincoln"
        },
        {
            "name": "The city of Saint Louis, nicknamed Gateway to the West, is located in which state?",
            "answers": [
                "Missouri",
                "Nebraska",
                "Mississippi",
                "Ohio"
            ],
            "correct": "Missouri"
        },
        {
            "name": "Malcolm X, the founder of the Organization of Afro-American Unity,  was born in what US city?",
            "answers": [
                "Kansas City",
                "Omaha",
                "Saint Louis",
                "Belleville"
            ],
            "correct": "Omaha"
        },
        {
            "name": "President George W. Bush was inaugurated on January 20, 2000.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "What is the capital of Illinois, the 21st US State?",
            "answers": [
                "Chicago",
                "Lincoln",
                "Springfield",
                "Omaha"
            ],
            "correct": "Springfield"
        },
        {
            "name": "This city, located in Colorado, is the highest city in the United States.",
            "answers": [
                "Vail",
                "Leadville",
                "Colorado Springs",
                "Denver"
            ],
            "correct": "Leadville"
        },
        {
            "name": "Amelia Earhart, the first woman to fly solo across the Atlantic, was born in what city and state?",
            "answers": [
                "Omaha, Nebraska",
                "Louisville, Kentucky",
                "New York City, New York",
                "Atchison, Kansas"
            ],
            "correct": "Atchison, Kansas"
        },
        {
            "name": "Arrange the following oceans by their total area, starting with the largest:",
            "answers": [
                "The Pacific Ocean",
                "The Indian Ocean",
                "The Arctic Ocean",
                "The Southern Ocean"
            ],
            "correct": "The Atlantic Ocean"
        },
        {
            "name": "In geography, the term desert is used to refer to a region that receives little or no precipitation. Approximately one-third of Earths land surface is desert, with the largest such region being which of these?",
            "answers": [
                "Sahara",
                "Antarctica",
                "Kalahari",
                "Greenland"
            ],
            "correct": "Antarctica"
        },
        {
            "name": "The Kalahari Desert, meaning great thirst translated from the local language, is a vast area of red-brown sands, taking up a considerable area of which of these regions?",
            "answers": [
                "Central Asia",
                "South Africa",
                "The Middle East",
                "Southwest Asia"
            ],
            "correct": "South Africa"
        },
        {
            "name": "This African desert, famous for its enormous sand dunes, is considered the oldest desert in the world, having endured its current arid conditions for at least 80 million years.",
            "answers": [
                "Libyan Desert",
                "Kalahari Desert",
                "Namib Desert",
                "Kara Kum Desert"
            ],
            "correct": "Namib Desert"
        },
        {
            "name": "Rangipo Desert in the central part of New Zealands North Island, does not have its barren and lifeless appearance because of the lack of rain, but rather, due to which of these?",
            "answers": [
                "Intense use of pesticides",
                "Constant powerful winds",
                "Soil structure",
                "Frequent destructive storms"
            ],
            "correct": "Soil structure"
        },
        {
            "name": "The Atacama Desert of Chile and Peru is one of the driest and most lifeless places on Earth. This virtually rainless plateau is made up of sand, lava flows, and what other substance?",
            "answers": [
                "Limestone",
                "Salt",
                "Clay",
                "Ash"
            ],
            "correct": "Salt"
        },
        {
            "name": "This Australian desert of nearly 600,000 square kilometres is famous for its spectacular land and rock formations, such as Uluru and Kata Tjuta, its estimated 1,100 parallel dunes running north-south, and the fresh water lake, Rainbow Valley.",
            "answers": [
                "Tanami Desert",
                "Simpson Desert",
                "Great Victoria Desert",
                "Great Sandy Desert"
            ],
            "correct": "Simpson Desert"
        },
        {
            "name": "The largest natural karst crater on Earth - Ramon Crater, with a length of 40 km (25 miles) and width of 2 to 10 km (1.5 to 6 miles), is situated in the heart of this Middle East desert.",
            "answers": [
                "Kyzyl Kum",
                "Negev desert",
                "Judean Desert",
                "Mojave desert"
            ],
            "correct": "Negev desert"
        },
        {
            "name": "Leonardo of Pisa or Leonardo Pisano, also known as Fibonacci, was an Italian mathematician and is best known for the discovery of the Fibonacci numbers, which form the following sequence. ",
            "answers": [
                " you know what the next number is?",
                "8",
                "10",
                "15"
            ],
            "correct": " 1, 1, 2, 3, 5, ..."
        },
        {
            "name": "Which one of these countries is largest in area?",
            "answers": [
                "Japan",
                "Jordan and Japan are equal in size",
                "Jordan",
                "Jamaica"
            ],
            "correct": "Japan"
        },
        {
            "name": "Which of the following oceans is the worlds largest?",
            "answers": [
                "Indian Ocean",
                "Arctic Ocean",
                "Atlantic Ocean",
                "Pacific Ocean"
            ],
            "correct": "Pacific Ocean"
        },
        {
            "name": "According to the United States Census Bureau in 2006, China was the most populated country in the world.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Which of the following countries is the worlds smallest continent, and can also be considered an island since it is surrounded by water?",
            "answers": [
                "India",
                "Africa",
                "Madagascar",
                "Australia"
            ],
            "correct": "Australia"
        },
        {
            "name": "Russia is the worlds largest country, with a land area of 17,098,242 square kilometers.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "This is the worlds smallest independent state with a population in 2007 of approximately 821.",
            "answers": [
                "Vatican City",
                "Monaco",
                "San Marino",
                "Tuvalu"
            ],
            "correct": "Vatican City"
        },
        {
            "name": "Which central European countrys all pre-1989 neighbors do not exist any more? The country is now bordered by 7 states.",
            "answers": [
                "Austria",
                "Poland",
                "Hungary",
                "Romania"
            ],
            "correct": "Poland"
        },
        {
            "name": "The African kingdom of Dahomey was powerful in the XVIII century. Which modern day country did it cover?",
            "answers": [
                "Burma",
                "Burkina Faso",
                "Benin",
                "DR Congo"
            ],
            "correct": "Benin"
        },
        {
            "name": "The Holy Roman Empire of the German Nation was a loosely coupled union of mostly German speaking principalities. Which part of the union was the driver of the 19th century unification of Germany?",
            "answers": [
                "Austria",
                "Prussia",
                "Saxony",
                "Bavaria"
            ],
            "correct": "Prussia"
        },
        {
            "name": "Where was the late medieval kingdom of Aragon situated?",
            "answers": [
                "Ireland and parts of Scotland",
                "Eastern Spain, major islands on the Mediterranean Sea and southern Italy",
                "Tolkiens fiction",
                "Syria, Turkey and northern Iraq"
            ],
            "correct": "Eastern Spain, major islands on the Mediterranean Sea and southern Italy"
        },
        {
            "name": "When Yugoslavia collapsed, one of its republics became an independent state known as FYROM. To which country does the acronym refer?",
            "answers": [
                "Albania",
                "Bosnia and Herzegovina",
                "Montenegro",
                "Macedonia"
            ],
            "correct": "Macedonia"
        },
        {
            "name": "By the end of the 16th century, a country called Commonwealth of Both Nations covered almost 1 million square kilometers of Europe. Which two countries formed it?",
            "answers": [
                "Sweden and Denmark",
                "Russia and Ukraine",
                "Poland and Lithuania",
                "Austria and Hungary"
            ],
            "correct": "Poland and Lithuania"
        },
        {
            "name": "Phoenicia was an ancient civilization that spread maritime trade over the Mediterranean Sea. From where did the Phoenicians originate?",
            "answers": [
                "North Africa (Tunis, Algier)",
                "Sicily (Syracuse)",
                "Greek islands on the Aegean Sea",
                "Modern day Liban (Sidon, Tyre)"
            ],
            "correct": "Modern day Liban (Sidon, Tyre)"
        },
        {
            "name": "Myanmar is a south-eastern Asian country. Under which name was it known before a military junta took over?",
            "answers": [
                "Siam",
                "Burma",
                "Bhutan",
                "Noth Vietnam"
            ],
            "correct": "Burma"
        },
        {
            "name": "The Ottoman Empire used to rule over North Africa, the Middle East and south-eastern Europe. Which modern day country is the successor of the empire?",
            "answers": [
                "Israel",
                "Saudi Arabia",
                "Turkey",
                "Egypt"
            ],
            "correct": "Turkey"
        },
        {
            "name": "Which independent country was not a Soviet republic?",
            "answers": [
                "Georgia",
                "Armenia",
                "Lithuania",
                "Albania"
            ],
            "correct": "Albania"
        },
        {
            "name": "Where was the monastic state of the Teutonic Knights located?",
            "answers": [
                "Seperated spots on modern day German territory",
                "Northern Poland, southern Lithuania, part of Latvia and the Russian enclave of Kaliningrad",
                "Southern France (Provance)",
                "Malta and other islands on the Mediterranean"
            ],
            "correct": "Northern Poland, southern Lithuania, part of Latvia and the Russian enclave of Kaliningrad"
        },
        {
            "name": "Where were the Boer Republics located?",
            "answers": [
                "In the Netherlands",
                "Across South Africa",
                "They were early Puritan settlements in the eastern North America",
                "Minor British isles"
            ],
            "correct": "Across South Africa"
        },
        {
            "name": "Finish this Irish saying: A tune is more lasting than the song of birds. And a word is more lasting than ________.",
            "answers": [
                "an old mans teeth",
                "the wealth of the world",
                "a maidens beauty",
                "a bottle of whiskey"
            ],
            "correct": "the wealth of the world"
        },
        {
            "name": "What is the missing word in this Irish saying: Every dog is _____ at his own house door.?",
            "answers": [
                "asleep",
                "lonely",
                "bold",
                "fearful"
            ],
            "correct": "bold"
        },
        {
            "name": "Complete this Irish saying: The whole world cannot make a ________ out of a donkey.",
            "answers": [
                "poet",
                "race horse",
                "nobleman",
                "artist"
            ],
            "correct": "race horse"
        },
        {
            "name": "Finish the following Irish saying: However long the day it ends with _______.",
            "answers": [
                "some bread",
                "a lullaby",
                "night",
                "a bang"
            ],
            "correct": "night"
        },
        {
            "name": "Complete this Irish saying: Dont let your ______ cut your throat.",
            "answers": [
                "jealousy",
                "mother-in-law",
                "tongue",
                "stale bread"
            ],
            "correct": "tongue"
        },
        {
            "name": "What is the missing word in this Irish saying: ______ is the poor mans doctor.?",
            "answers": [
                "An apple",
                "Socialism",
                "Death",
                "Over-the-counter medication"
            ],
            "correct": "Death"
        },
        {
            "name": "Complete the following Irish saying: If you want ______, always carry the back tooth of a horse with you.",
            "answers": [
                "to go swimming",
                "beauty",
                "money",
                "to be the life of the party"
            ],
            "correct": "money"
        },
        {
            "name": "Finish this Irish saying: If someone is sick turn his bed so that it faces _________.",
            "answers": [
                "the TV",
                "the wall",
                "north",
                "a window"
            ],
            "correct": "north"
        },
        {
            "name": "Finish this Irish saying: The far off hills are _______.",
            "answers": [
                "covered with clouds",
                "alive with the sound of music",
                "the farthest away",
                "the greenest"
            ],
            "correct": "the greenest"
        },
        {
            "name": "Finish this Irish saying: Often has the likely failed and the unlikely _________.",
            "answers": [
                "failed too",
                "wrote a novel about the failure",
                "laughed at the failure",
                "succeeded"
            ],
            "correct": "succeeded"
        },
        {
            "name": "What is the capital and largest city of Jamaica?",
            "answers": [
                "Montego Bay",
                "Kigali",
                "Harare",
                "Kingston"
            ],
            "correct": "Kingston"
        },
        {
            "name": "What is the name of the capital city of India?",
            "answers": [
                "Jaipur",
                "Mumbai",
                "Calcutta",
                "New Delhi"
            ],
            "correct": "New Delhi"
        },
        {
            "name": "Tegucigalpa is the capital and largest city of this country.",
            "answers": [
                "Namibia",
                "Honduras",
                "Uzbekistan",
                "Laos"
            ],
            "correct": "Honduras"
        },
        {
            "name": "What is the capital and largest city of Romania?",
            "answers": [
                "Ankara",
                "Sofia",
                "Kiev",
                "Bucharest"
            ],
            "correct": "Bucharest"
        },
        {
            "name": "Addis Ababa is the capital of what country?",
            "answers": [
                "Samoa",
                "Paraguay",
                "Ghana",
                "Ethiopia"
            ],
            "correct": "Ethiopia"
        },
        {
            "name": "What is the capital and largest city of Lebanon?",
            "answers": [
                "Bujumbura",
                "Dhaka",
                "Baghdad",
                "Beirut"
            ],
            "correct": "Beirut"
        },
        {
            "name": "Kathmandu is the capital city of what country?",
            "answers": [
                "Gabon",
                "Nepal",
                "Togo",
                "Rwanda"
            ],
            "correct": "Nepal"
        },
        {
            "name": "What is the name of the capital city of Latvia?",
            "answers": [
                "Reykjavik",
                "Port Vila",
                "Porto-Novo",
                "Riga"
            ],
            "correct": "Riga"
        },
        {
            "name": "This city is the capital of the Kingdom of Morocco.",
            "answers": [
                "Suva",
                "Yaounde",
                "Tehran",
                "Rabat"
            ],
            "correct": "Rabat"
        },
        {
            "name": "The capital city of what country is called Quito?",
            "answers": [
                "Bolivia",
                "Macedonia",
                "Ecuador",
                "Yemen"
            ],
            "correct": "Ecuador"
        },
        {
            "name": "Name Zimbabwes capital which is an independent city equivalent to a province.",
            "answers": [
                "Harare",
                "Cairo",
                "Rome",
                "Tunis"
            ],
            "correct": "Harare"
        },
        {
            "name": "Name Moroccos capital that is also the capital of the Rabat-Salé-Zemmour-Zaer region.",
            "answers": [
                "Casablanca",
                "Pretoria",
                "Niamey",
                "Rabat"
            ],
            "correct": "Rabat"
        },
        {
            "name": "Bamako is the capital of Madagascar.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "False"
        },
        {
            "name": "What is the capital and largest city of the Republic of the Congo?",
            "answers": [
                "Brazzaville",
                "Lusaka",
                "Lima",
                "Libreville"
            ],
            "correct": "Brazzaville"
        },
        {
            "name": "Ethiopias capital is Addis Ababa.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What port on the Atlantic Ocean is the capital and largest city of Guinea.",
            "answers": [
                "Baku",
                "Dhaka",
                "Conakry",
                "Bujumbura"
            ],
            "correct": "Conakry"
        },
        {
            "name": "What is the capital and largest commercial city of Libya?",
            "answers": [
                "Ethiopia",
                "Windhoek",
                "Nouakchott",
                "Tripoli"
            ],
            "correct": "Tripoli"
        },
        {
            "name": "This city is Kenyas capital and the most populous city in East Africa.",
            "answers": [
                "Cape Town",
                "Nairobi",
                "Port Louis",
                "Tripoli"
            ],
            "correct": "Nairobi"
        },
        {
            "name": "Victoria is the capital of the Republic of Seychelles.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Alaska used to be part of Russia.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "What is the currency of Russia?",
            "answers": [
                "franc",
                "ruble",
                "dollar",
                "peso"
            ],
            "correct": "ruble"
        },
        {
            "name": "How do you say hello in Russian?",
            "answers": [
                "Spasiba",
                "Zdravstvuite",
                "Dobry den",
                "Do svidanya"
            ],
            "correct": "Zdravstvuite"
        },
        {
            "name": "What phrase means I love you in Russian?",
            "answers": [
                "Eu te amo",
                "Ya lyublyu tebya",
                "Te iu besc",
                "Volim te"
            ],
            "correct": "Ya lyublyu tebya"
        },
        {
            "name": "Tetris, the popular falling-blocks video game, was invented in Russia.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Who is the first President of the Russian Federation?",
            "answers": [
                "Mikhail Kasyanov",
                "Viktor Khristenko",
                "Boris Yeltsin",
                "Vladimir Putin"
            ],
            "correct": "Boris Yeltsin"
        },
        {
            "name": "The first person in space was Russian.",
            "answers": [
                "False",
                "True",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "Who commissioned  the St. Basils Cathedral?",
            "answers": [
                "Ivan the Marvelous",
                "Ivan the Terrible",
                "Ivan the Ultimate",
                "Ivan the Magnificent"
            ],
            "correct": "Ivan the Terrible"
        },
        {
            "name": "Name the capital city of the U.S. state of Wyoming.",
            "answers": [
                "Boise",
                "Des Moines",
                "Salt Lake City",
                "Cheyenne"
            ],
            "correct": "Cheyenne"
        },
        {
            "name": "The U.S. state of Oregon has been nicknamed after this animal, which is the official state animal.",
            "answers": [
                "Raccoon",
                "Beaver",
                "Marmot",
                "Bear"
            ],
            "correct": "Beaver"
        },
        {
            "name": "Which U.S. state has been nicknamed The Silver State?",
            "answers": [
                "Nevada",
                "Nebraska",
                "North Dakota",
                "New Mexico"
            ],
            "correct": "Nevada"
        },
        {
            "name": "Albuquerque is the largest city of which U.S. state?",
            "answers": [
                "Ohio",
                "Kansas",
                "Oklahoma",
                "New Mexico"
            ],
            "correct": "New Mexico"
        },
        {
            "name": "Name the capital city of the U.S. state of Montana.",
            "answers": [
                "Helena",
                "Billings",
                "Pierre",
                "Sioux Falls"
            ],
            "correct": "Helena"
        },
        {
            "name": "What does the name of the U.S. state of Texas literally mean in Caddo?",
            "answers": [
                "Vast",
                "Buffalo",
                "Wilderness",
                "Friends"
            ],
            "correct": "Friends"
        },
        {
            "name": "The state of Louisiana is bordered by the Gulf of Mexico on the south and three states. Which state does NOT border Louisiana?",
            "answers": [
                "Oklahoma",
                "Texas",
                "Arkansas",
                "Mississippi"
            ],
            "correct": "Oklahoma"
        },
        {
            "name": "When René-Robert Cavelier, Sieur de La Salle (Yikes!) claimed the territory he named it La Louisiane, which means....",
            "answers": [
                "Land of Lords or Land of the Lords.",
                "Land of France or Long Live France.",
                "Land of Life or The Life.",
                "Land of Louis or The Lousiana."
            ],
            "correct": "Land of Louis or The Lousiana."
        },
        {
            "name": "What is the closest definition of Santeria?",
            "answers": [
                "Another name for the Catholic Church in Lousiana.",
                "Another name for the Protestant Baptists religion in Lousiana.",
                "A Cuban based religion popular in Lousiana.",
                "A Jewish based religion popular in Lousiana."
            ],
            "correct": "A Cuban based religion popular in Lousiana."
        },
        {
            "name": "The first recorded Euroregion, Gronau, was founded in 1958. What countries established it?",
            "answers": [
                "Austria and Switzerland",
                "Germany and Austria",
                "Germany and Netherlands",
                "Switzerland and Germany"
            ],
            "correct": "Germany and Netherlands"
        },
        {
            "name": "The Beskidy (Beskydy) Euroregion is located in the west part of the Carpathian mountain range. Which three countries does it span?",
            "answers": [
                "Hungary, Romania, Ukraine",
                "Czech Republic, Poland, Slovakia",
                "Czech Republic, Austria, Slovakia",
                "Poland, Slovakia, Ukraine"
            ],
            "correct": "Czech Republic, Poland, Slovakia"
        },
        {
            "name": "Which country is not a member of the Adriatic Euroregion?",
            "answers": [
                "Montenegro",
                "Albania",
                "Serbia",
                "Italy"
            ],
            "correct": "Serbia"
        },
        {
            "name": "The ore and coal rich region of Silesia (pol.: Ślask, ger.: Schliesien, cz.: Slezsko) was ruled by Germans, Poles, Czechs and Austrians. To which country does it mostly belong today?",
            "answers": [
                "Slovakia",
                "Germany",
                "Poland",
                "Czech Republic"
            ],
            "correct": "Poland"
        },
        {
            "name": "Where would you find the Tornio River Valley Euroregion?",
            "answers": [
                "Sweden / Finland",
                "Italy / Slovenia",
                "Italy / France",
                "Austria / Italy"
            ],
            "correct": "Sweden / Finland"
        },
        {
            "name": "The Cross Channel Euroregion includes France, the UK and what other country?",
            "answers": [
                "Belgium",
                "Ireland",
                "Spain",
                "Netherlands"
            ],
            "correct": "Belgium"
        },
        {
            "name": "The region of Pomerania (ger.: Pomern, pol.: Pomorze) is located in north Germany and Poland. Which river divides the region into two parts?",
            "answers": [
                "Vistula",
                "Danube",
                "Elbe",
                "Oder (Odra)"
            ],
            "correct": "Oder (Odra)"
        },
        {
            "name": "The TriRhena Euroregion is spans these three countries crossed by the Rhine River.",
            "answers": [
                "France, Germany and Switzerland",
                "Belgium, Germany and Luxembourg",
                "Austria, Germany and Switzerland",
                "France, Germany and Holland"
            ],
            "correct": "France, Germany and Switzerland"
        },
        {
            "name": "The Pyrenees-Mediterranean region is located along the border of which two countries?",
            "answers": [
                "France - Italy",
                "Norway - Sweden",
                "France - Spain",
                "Spain - Portugal"
            ],
            "correct": "France - Spain"
        },
        {
            "name": "Where would you find the Ore Mountains Euroregion?",
            "answers": [
                "Greece and Bulgaria",
                "Spain and France",
                "Czech Republic and Germany",
                "Norway and Sweden"
            ],
            "correct": "Czech Republic and Germany"
        },
        {
            "name": "What is the state animal of Colorado and the symbol of Rocky Mountain National Park?",
            "answers": [
                "Mustang",
                "Big Horned Sheep",
                "White Tailed Deer",
                "Buffalo"
            ],
            "correct": "Big Horned Sheep"
        },
        {
            "name": "Was Isabella Bird the first woman to climb Longs Peak, the highest point in Rocky Mountain National Park?",
            "answers": [
                "No",
                "Yes",
                null,
                null
            ],
            "correct": "No"
        },
        {
            "name": "Mesa Verde National Park in southern Colorado is best known for which of the following?",
            "answers": [
                "Rather large, fossilized dinosaur tracks",
                "Massive trees known as Sequoias",
                "The best-preserved cliff dwellings in the U.S.",
                "A very rare flower that only grows in the mountains"
            ],
            "correct": "The best-preserved cliff dwellings in the U.S."
        },
        {
            "name": "One the worlds largest silver nuggets was found in Colorado.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The Colorado River, the major river of the American southwest, flows through which national landmark?",
            "answers": [
                "The Hoover Dam",
                "The Grand Canyon",
                "Arches National Park",
                "All of these"
            ],
            "correct": "All of these"
        },
        {
            "name": "Located at the Peterson Air Force base in Colorado, NORAD is a military organization that tracks aircraft, missiles and space vehicles. It also tracks what fictional character?",
            "answers": [
                "The Easter Bunny",
                "The Tooth Fairy",
                "None of these",
                "Santa"
            ],
            "correct": "Santa"
        },
        {
            "name": "What U.S. interstate runs east and west through Colorado?",
            "answers": [
                "I-40",
                "I-80",
                "I-90",
                "I-70"
            ],
            "correct": "I-70"
        },
        {
            "name": "Today people visit Colorado to go skiing, hiking, mountain biking and to enjoy the scenery. But 100 years ago, tourists more often went to the mountains to do what?",
            "answers": [
                "Soak in hot springs",
                "Play golf",
                "Record music",
                "Go horseback riding"
            ],
            "correct": "Soak in hot springs"
        },
        {
            "name": "Early Colorado ranchers raised cattle, sheep, hogs, chickens, and turkeys, with the main focus on cattle.",
            "answers": [
                "True",
                "False",
                null,
                null
            ],
            "correct": "True"
        },
        {
            "name": "The huge grain industry in Colorado gives rise to breweries. Which of the following owns the biggest single site brewery located in Golden, Colorado?",
            "answers": [
                "Anheuser-Busch",
                "Molson Coors Brewing Company",
                "Jacob Leinenkugel Brewing Company",
                "Highfalls Brewing Company"
            ],
            "correct": "Molson Coors Brewing Company"
        },
        {
            "name": "On May 5, 1920 two Italian-Americans, Nicola Sacco and Bartolomeo Vanzetti, were arrested, tried, and executed for the April 15, 1920 murders of a South Braintree paymaster and security guard and theft of the payroll. ",
            "answers": [
                "He was at the Italian consulate in Boston.",
                "He was in Plymouth peddling fish at the time.",
                "He was visiting friends in West Bridgewater.",
                "He was at the Italian consulate in Boston."
            ],
            "correct": "at did Sacco give as his alibi?"
        },
        {
            "name": "What is the state motto of Colorado?",
            "answers": [
                "Nothing Without the Deity",
                "The people rule",
                "God enriches",
                "We Dare Defend Our Rights"
            ],
            "correct": "Nothing Without the Deity"
        },
        {
            "name": "What does the word Colorado mean?",
            "answers": [
                "Colored Red",
                "Indian Sky",
                "Rocky Mountains",
                "Mountain High"
            ],
            "correct": "Colored Red"
        },
        {
            "name": "What is the state fish of Colorado?",
            "answers": [
                "Rainbow Trout",
                "Greenback Cutthroat Trout",
                "Walleye",
                "Salmon"
            ],
            "correct": "Greenback Cutthroat Trout"
        },
        {
            "name": "What city in Colorado is known as the Richest Square Mile on Earth?",
            "answers": [
                "Central City, Colorado",
                "Denver, Colorado",
                "Leadville, Colorado",
                "Colorado Springs, Colorado"
            ],
            "correct": "Central City, Colorado"
        },
        {
            "name": "What was the popular slogan of the Colorado Gold Rush?",
            "answers": [
                "Westward Ho",
                "Colorado Gold to Go",
                "Go West Young Men",
                "Pikes Peak or Bust"
            ],
            "correct": "Pikes Peak or Bust"
        },
        {
            "name": "What year did Colorado become a state?",
            "answers": [
                "1876",
                "1796",
                "1896",
                "1776"
            ],
            "correct": "1876"
        },
        {
            "name": "What city boasts the most parks in the US?",
            "answers": [
                "Colorado Springs, Colorado",
                "Golden, Colorado",
                "Denver, Colorado",
                "Aspen, Colorado"
            ],
            "correct": "Denver, Colorado"
        },
        {
            "name": "What legendary American frontiersman commanded a major fort in Colorado?",
            "answers": [
                "Daniel Boone",
                "Kit Carson",
                "Wild Bill Cody",
                "Jim Bowie"
            ],
            "correct": "Kit Carson"
        },
        {
            "name": "Glenwood Springs, Colorado was known for its hot springs and hospital. What famous outlaw and gun slinger expired from Tuberculosis there?",
            "answers": [
                "Billy The Kid",
                "Jesse James",
                "Doc Holliday",
                "Wyatt Earp"
            ],
            "correct": "Doc Holliday"
        },
        {
            "name": "On which continent is the famous active volcano Mount Nyiragongo located?",
            "answers": [
                "Asia",
                "Australia",
                "South America",
                "Africa"
            ],
            "correct": "Africa"
        },
        {
            "name": "On which of the islands of Papua New Guinea is the active volcano Ulawun located?",
            "answers": [
                "New Britain",
                "New Guinea",
                "Bougainville",
                "New Ireland"
            ],
            "correct": "New Britain"
        },
        {
            "name": "The Arabic name of Mount Etna, the second-largest active volcano in Europe, was Jebel Utlamat. What does this mean in English?",
            "answers": [
                "The Edge of the World",
                "Mountain of Fire",
                "Home of the Devil",
                "Breathing Mountain"
            ],
            "correct": "Mountain of Fire"
        },
        {
            "name": "In what country is the active volcano Sakurajima located?",
            "answers": [
                "India",
                "Peru",
                "Japan",
                "South Africa"
            ],
            "correct": "Japan"
        },
        {
            "name": "What is the name of the most active volcano in Mexico, located mostly in the Mexican state of Jalisco?",
            "answers": [
                "Nevado de Toluca",
                "Barcena",
                "Colima",
                "Las Cumbres"
            ],
            "correct": "Colima"
        },
        {
            "name": "What does the Indonesian name of the Mount Merapi volcano, Gunung Merapi, mean in English?",
            "answers": [
                "Gods Wrath",
                "Cursed Mountain",
                "Mountain of Fire",
                "Hell Mountain"
            ],
            "correct": "Mountain of Fire"
        },
        {
            "name": "In what country is the active volcano Koryaksky located?",
            "answers": [
                "Russia",
                "Ukraine",
                "Latvia",
                "Poland"
            ],
            "correct": "Russia"
        },
        {
            "name": "In what US state is the Fourpeaked Volcano, also known as Fourpeaked Mountain, situated?",
            "answers": [
                "Colorado",
                "Oregon",
                "Alaska",
                "Montana"
            ],
            "correct": "Alaska"
        },
        {
            "name": "What is the name of the highest volcano in Asia, and the highest point in the Middle East?",
            "answers": [
                "Bazman",
                "Sabalan",
                "Taftan",
                "Mount Damavand"
            ],
            "correct": "Mount Damavand"
        },
        {
            "name": "What is the name of the active volcano situated in Skamania County, Washington, USA, 50 miles (80 km) northeast of Portland, Oregon?",
            "answers": [
                "Mount St. Augustus",
                "Mount St. Paul",
                "Mount St. Georges",
                "Mount St. Helens"
            ],
            "correct": "Mount St. Helens"
        },
        {
            "name": "The self-governed region of the Holy Mountain is considered part of the Greek state according to a decree",
            "answers": [
                "1913",
                "1910",
                "1913",
                "1925"
            ],
            "correct": "ssed in what year?"
        },
        {
            "name": "What type of government does Sweden have?",
            "answers": [
                "Absolute monarchy",
                "Constitutional monarchy",
                "None of these",
                "Presidential republic"
            ],
            "correct": "Constitutional monarchy"
        },
        {
            "name": "Which of these European countries shares a land border with Sweden?",
            "answers": [
                "Switzerland",
                "Denmark",
                "Lithuania",
                "Norway"
            ],
            "correct": "Norway"
        },
        {
            "name": "What are the colors of the national flag of Sweden?",
            "answers": [
                "Blue and yellow",
                "White and blue",
                "White and red",
                "Red and yellow"
            ],
            "correct": "Blue and yellow"
        },
        {
            "name": "What is the official language spoken by the citizens of Sweden?",
            "answers": [
                "Finnish and Swedish",
                "Swedish",
                "German and Swedish",
                "Danish"
            ],
            "correct": "Swedish"
        },
        {
            "name": "What is the name of the largest city and capital of Sweden?",
            "answers": [
                "Helsinki",
                "Stockholm",
                "Oslo",
                "Gothenburg"
            ],
            "correct": "Stockholm"
        },
        {
            "name": "The Swedish name of Sweden is Sverige. What does this word literally mean?",
            "answers": [
                "Snow kingdom",
                "Kingdom of the Swedes",
                "Land of snow",
                "Northern land"
            ],
            "correct": "Kingdom of the Swedes"
        },
        {
            "name": "Which of these celebrities is not Swedish?",
            "answers": [
                "the members of ABBA",
                "Lars Ulrich",
                "Greta Garbo",
                "Ingrid Bergman"
            ],
            "correct": "Lars Ulrich"
        },
        {
            "name": "What type of food is the popular Swedish dish called Surstromming?",
            "answers": [
                "Fish",
                "Salad",
                "Soup",
                "Dessert"
            ],
            "correct": "Fish"
        },
        {
            "name": "What song is used as a de facto anthem of Sweden?",
            "answers": [
                "Beautiful Sweden",
                "The Song of the King",
                "We Bow Before Thee",
                "Thou Ancient, Thou Free"
            ],
            "correct": "Thou Ancient, Thou Free"
        },
        {
            "name": "What is the prevailing religion in Sweden?",
            "answers": [
                "Lutheranism",
                "Eastern Orthodoxy",
                "Protestantism",
                "Roman Catholicism"
            ],
            "correct": "Lutheranism"
        },
        {
            "name": "What is the capital city of the Federative Republic of Brazil?",
            "answers": [
                "Sao Paulo",
                "Bogota",
                "Rio de Janeiro",
                "Brasilia"
            ],
            "correct": "Brasilia"
        },
        {
            "name": "What is the meaning of the Portuguese word brasa from which the name of Brazil possibly originated?",
            "answers": [
                "Jungle",
                "River",
                "Heaven",
                "Ember"
            ],
            "correct": "Ember"
        },
        {
            "name": "Which of these statements is wrong about the wildlife of Brazil?",
            "answers": [
                "It contains the most mammal species on the planet.",
                "Brazil is the country with the largest biodiversity on the planet.",
                "All of these",
                "Brazil is the country with the most endemic species in the world."
            ],
            "correct": "Brazil is the country with the most endemic species in the world."
        },
        {
            "name": "What are the colors of the national flag of the Federative Republic of Brazil?",
            "answers": [
                "Yellow, green, black and blue",
                "White, blue, green and yellow",
                "Red, green and yellow",
                "Green and black"
            ],
            "correct": "White, blue, green and yellow"
        },
        {
            "name": "What is the official language of the Federative Republic of Brazil?",
            "answers": [
                "English and Spanish",
                "Spanish",
                "Spanish and Portuguese",
                "Portuguese"
            ],
            "correct": "Portuguese"
        },
        {
            "name": "What is the most popular sport among the people living in Brazil?",
            "answers": [
                "Volleyball",
                "Rugby",
                "Basketball",
                "Football"
            ],
            "correct": "Football"
        },
        {
            "name": "What is the largest and richest city in Brazil?",
            "answers": [
                "Sao Paulo",
                "Salvador",
                "Rio de Janeiro",
                "Brasilia"
            ],
            "correct": "Sao Paulo"
        },
        {
            "name": "What is the national motto of the Federative Republic of Brazil, also featured in the national flag of the country?",
            "answers": [
                "Faith and Honesty",
                "Freedom Above All",
                "Order and Progress",
                "Unity and Strength"
            ],
            "correct": "Order and Progress"
        },
        {
            "name": "What type of food is the traditional Brazilian dish called Moqueca?",
            "answers": [
                "Dessert",
                "Drink",
                "Salad",
                "Stew"
            ],
            "correct": "Stew"
        },
        {
            "name": "On what day of the week does the parade of the famous Rio Carnival traditionally start?",
            "answers": [
                "Sunday",
                "Thursday",
                "Wednesday",
                "Friday"
            ],
            "correct": "Sunday"
        }
    ]
}
"#;
