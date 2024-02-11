/*
 * This program reads the Iris dataset from a CSV file located at "src/data/iris.csv".
 * It assumes the CSV has headers.
 *
 * 1. Filtering:
 *    It filters the rows where the column 'sepal_length' is greater than 5.0.
 *
 * 2. Grouping:
 *    The program then groups the data by the 'species' column.
 *
 * 3. Aggregation:
 *    For each group (species in this case), it calculates the sum of the other columns.
 *
 * Output Explanation:
 *    The resulting DataFrame has a shape of (3, 5), indicating that it has 3 rows and 5 columns.
 *    The rows represent the three species of Iris flowers: 'setosa', 'versicolor', and 'virginica'.
 *    The columns represent:
 *    - 'species': The name of the Iris species
 *    - 'sepal_length': The sum of 'sepal_length' values for each species
 *    - 'sepal_width': The sum of 'sepal_width' values for each species
 *    - 'petal_length': The sum of 'petal_length' values for each species
 *    - 'petal_width': The sum of 'petal_width' values for each species
 *
 *    For example, for 'virginica', the sum of 'sepal_length' values is 324.5,
 *    the sum of 'sepal_width' values is 146.2, and so on.
 */

// Import necessary modules from the `polars` crate
use polars::prelude::*;
use std::io::Cursor;

// This is the Iris dataset in CSV format.

const SALARIES_DATA: &str = "first_name,last_name,team,position,salary
Luis,Abram,Atlanta United,D,556364.00
Lalas,Abubakar,Colorado Rapids,D,665000.00
Nicol√°s,Acevedo,New York City FC,D-M,230000.00
Alonso,Aceves,Chicago Fire,D,330000.00
Ifunanyachi,Achara,Houston Dynamo,F,85444.00
Kellyn,Acosta,LAFC,D-M,1250000.00
Bryan,Acosta,Portland Timbers,M,675000.00
Luciano,Acosta,FC Cincinnati,M-F,1943500.00
Sam,Adekugbe,Vancouver Whitecaps,D,729516.00
Samuel,Adeniran,St. Louis City SC,F,85444.00
Harrison,Afful,Charlotte FC,D,85444.00
Stephen,Afrifa,Sporting Kansas City,F,67360.00
Willy,Agada,Sporting Kansas City,F,225000.00
London,Aghedo,FC Cincinnati,D,67360.00
Oskar,Agren,San Jose Earthquakes,D,85444.00
Daniel,Aguirre,LA Galaxy,M,85444.00
Patrick,Agyemang,Charlotte FC,F,67360.00
Ali,Ahmed,Vancouver Whitecaps,M,85444.00
Carlos,Akapo,San Jose Earthquakes,D,525000.00
Matai,Akinmboni,DC United,D,67360.00
Ayo,Akinola,San Jose Earthquakes,F,700000.00
Joey,Akpunonu,FC Cincinnati,D,67360.00
Jordi,Alba,Inter Miami,D,1000000.00
Caio,Alexandre Souza e Silva,Vancouver Whitecaps,M,500000.00
Tony,Alfaro,LA Galaxy,D,120000.00
Ibrahim,Aliyu,Houston Dynamo,F,425004.00
Noah,Allen,Inter Miami,D,67360.00
Rasmus,Alm,St. Louis City SC,M,220000.00
Thiago,Almada,Atlanta United,M-F,1650000.00
Osvaldo,Alonso,Atlanta United,D-M,88200.00
Jozy,Altidore,Major League Soccer,F,2381139.00
Fernando,√Ålvarez,CF Montreal,D,85444.00
Leonardo,Alves Ch√∫ Franco,Seattle Sounders FC,M,550000.00
Max,Alves da Silva,Colorado Rapids,M-F,232000.00
Antony,Alves Santos,Portland Timbers,M,425000.00
Frankie,Amaya,New York Red Bulls,M-F,350000.00
Kemy,Amiche,Nashville SC,M,67360.00
Malte,Amundsen,Columbus Crew,D,300000.00
Max,Anchor,Vancouver Whitecaps,GK,67360.00
Sebastian,Anderson,Colorado Rapids,D,120000.00
Iv√°n,Angulo,Orlando City SC,M,348000.00
Marco,Angulo,FC Cincinnati,D-M,275000.00
Eugene,Ansah,FC Dallas,M,200000.00
Themi,Antonoglou,Toronto FC,D,68775.00
Tah,Anunga,Nashville SC,M,250000.00
C√©sar,Ara√∫jo,Orlando City SC,D-M,255000.00
H√©ber,Ara√∫jo dos Santos,Seattle Sounders FC,F,930000.00
Cristian,Arango,Real Salt Lake,F,1899996.00
Miguel,Araujo,Portland Timbers,D,525000.00
Scott,Arfield,Charlotte FC,M,400000.00
Max,Arfsten,Columbus Crew,M,67360.00
Santiago,Arias,FC Cincinnati,D,550000.00
Adam,Armour,Major League Soccer,D,85444.00
Xavier,Arreaga,Seattle Sounders FC,D,750000.00
Kervin,Arriaga,Minnesota United,D-M,170000.00
Paul,Arriola,FC Dallas,M,1550000.00
Dixon,Arroyo,Inter Miami,D-M,150000.00
Yamil,Asad,DC United,M,100000.00
Charlie,Asensio,Austin FC,D,68775.00
Thor,√ölfarsson,Houston Dynamo,M-F,85444.00
Dairon,Asprilla,Portland Timbers,M,365000.00
Jean-Aniel,Assi,CF Montreal,M,85444.00
Josh,Atencio,Seattle Sounders FC,D-M,92400.00
Juli√°n,Aude,LA Galaxy,D,275000.00
Charles,Auguste,Houston Dynamo,M,67360.00
Beto,Avila,Houston Dynamo,F,67360.00
Tom√°s,Avil√©s,Inter Miami,D,360000.00
David,Ayala,Portland Timbers,D-M,375000.00
Edison,Azcona,Inter Miami,M,85444.00
Roman,B√ºrki,St. Louis City SC,GK,1500000.00
Talles Magno,Bacelar Martins,New York City FC,M,950000.00
Dominique,Badji,FC Cincinnati,F,550000.00
Drew,Baiera,New York City FC,D,67360.00
Corey,Baird,Houston Dynamo,F,550000.00
Esmir,Bajraktarevic,New England Revolution,M,85444.00
Cody,Baker,Seattle Sounders FC,D,67360.00
Reed,Baker-Whiting,Seattle Sounders FC,M,85444.00
Monsef,Bakrar,New York City FC,F,336000.00
Michael,Baldisimo,San Jose Earthquakes,D-M,85444.00
Esequiel,Barco,Atlanta United,M,2200000.00
Tai,Baribo,Philadelphia Union,F,600000.00
Tom,Barlow,New York Red Bulls,F,195000.00
Luis,Barraza,New York City FC,GK,145000.00
√Ålvaro,Barreal,FC Cincinnati,M,300000.00
Michael,Barrios,LA Galaxy,M,650000.00
Lucas,Bartlett,St. Louis City SC,D,85444.00
Ethan,Bartlow,Houston Dynamo,D,125000.00
Cole,Bassett,Colorado Rapids,M-F,450000.00
Amine,Bassi,Houston Dynamo,M-F,550000.00
Josh,Bauer,Nashville SC,D,89716.00
Tanner,Beason,San Jose Earthquakes,D,250000.00
Gavin,Beavers,Real Salt Lake,GK,85444.00
Simon,Becher,Vancouver Whitecaps,F,67360.00
Alejandro,Bedoya,Philadelphia Union,M,780000.00
Steven,Beitashour,Colorado Rapids,D,150000.00
Jon,Bell,St. Louis City SC,D,88200.00
Nico,Benalc√°zar,New York City FC,D,68775.00
Ben,Bender,Charlotte FC,M,85444.00
Joe,Bendik,Philadelphia Union,GK,175000.00
Julio,Benitez,Real Salt Lake,D-M,85444.00
Christian,Benteke,DC United,F,4250000.00
Nimfasha,Berchimas,Charlotte FC,M,67360.00
Sebastian,Berhalter,Vancouver Whitecaps,M,150000.00
Federico,Bernardeschi,Toronto FC,M,3125000.00
Miguel,Berry,Atlanta United,F,135000.00
Matt,Bersano,Austin FC,GK,125000.00
Victor,Bezerra,Chicago Fire,F,85444.00
David,Bingham,Portland Timbers,GK,85444.00
Steve,Birnbaum,DC United,D,815000.00
Stipe,Biuk,LAFC,M,650000.00
Tristan,Blackmon,Vancouver Whitecaps,D,400000.00
Andre,Blake,Philadelphia Union,GK,775000.00
Sebasti√°n,Blanco,Portland Timbers,M-F,1380000.00
Latif,Blessing,Toronto FC,M-F,600000.00
Njabulo,Blom,St. Louis City SC,D-M,246000.00
Emmanuel,Boateng,New England Revolution,M,85444.00
Isaac,Boehmer,Vancouver Whitecaps,GK,68775.00
Mateusz,Bogusz,LAFC,M-F,650000.00
Franck,Boli,Portland Timbers,F,450000.00
Josh,Bolma,New England Revolution,M,67360.00
Mo√Øse,Bombito,Colorado Rapids,D,67360.00
Jonathan,Bond,LA Galaxy,GK,475000.00
Pablo,Bonilla,Major League Soccer,D,135000.00
Alex,Bono,DC United,GK,85444.00
Shanyder,Borgelin,Inter Miami,F,67360.00
Dylan,Borrero,New England Revolution,M,575000.00
Gustavo,Bou,New England Revolution,F,2500000.00
Denis,Bouanga,LAFC,M,2083333.00
Ousseni,Bouda,San Jose Earthquakes,M,100000.00
Aaron,Boupendza,FC Cincinnati,F,720000.00
Michael,Boxall,Minnesota United,D,650000.00
Tyler,Boyd,LA Galaxy,M,275000.00
Michael,Bradley,Toronto FC,M,614286.00
Chris,Brady,Chicago Fire,GK,189996.00
Zachary,Brault-Guillard,CF Montreal,D,275000.00
Claudio,Bravo,Portland Timbers,D,315000.00
Ethan,Bristow,Minnesota United,D,85444.00
Andrew,Brody,Real Salt Lake,D,250000.00
Brandt,Bronico,Charlotte FC,M,310000.00
Javain,Brown,Vancouver Whitecaps,D,140000.00
Gast√≥n,Brugman,LA Galaxy,D-M,1200000.00
Will,Bruin,Austin FC,F,200000.00
Noel,Buck,New England Revolution,M,67360.00
Jes√∫s,Bueno,Philadelphia Union,D-M,170000.00
Teal,Bunbury,Nashville SC,F,425000.00
Nkosi Tafari,Burgess,FC Dallas,D,240750.00
Cory,Burke,New York Red Bulls,F,500000.00
Kendall,Burks,Chicago Fire,D,88200.00
Evan,Bush,Columbus Crew,GK,88200.00
Sergio,Busquets,Inter Miami,D-M,1500000.00
Brandon,Bye,New England Revolution,D,350000.00
Nathan,Byrne,Charlotte FC,D,560000.00
Mart√≠n,C√°ceres,LA Galaxy,D,500004.00
Jorge,Cabezas,New York Red Bulls,F,85444.00
K√©vin,Cabral,Colorado Rapids,M,1800000.00
D√©iber,Caicedo,Vancouver Whitecaps,M,350000.00
Luis,Caicedo,Houston Dynamo,D-M,110000.00
Scott,Caldwell,Real Salt Lake,M,225000.00
Lucas Felipe,Calegari,LA Galaxy,D,550000.00
Noel,Caliskan,Portland Timbers,M,67360.00
Drake,Callender,Inter Miami,GK,300000.00
Rudy,Camacho,Columbus Crew,D,500000.00
Brandon,Cambridge,Charlotte FC,M,67360.00
Matteo,Campagna,Vancouver Whitecaps,D,85444.00
Leonardo,Campana,Inter Miami,F,549996.00
George,Campbell,CF Montreal,D,325000.00
Russell,Canouse,DC United,M,575000.00
Wikelman,Carmona,New York Red Bulls,M-F,85444.00
Juli√°n,Carranza,Philadelphia Union,F,950000.00
Adalberto,Carrasquilla,Houston Dynamo,M,500000.00
Antonio,Carrera,FC Dallas,GK,85444.00
M√°ximo,Carrizo,New York City FC,M-F,67360.00
Wilder,Cartagena,Orlando City SC,D-M,348000.00
Javier,Casas,Chicago Fire,M,85444.00
Julio,Cascante,Austin FC,D,620000.00
Robert,Castellanos,Sporting Kansas City,D,85444.00
Jacob,Castro,Seattle Sounders FC,GK,67360.00
Roman,Celentano,FC Cincinnati,GK,100000.00
Erik,Centeno,Atlanta United,D,67360.00
Edwin,Cerrillo,LA Galaxy,D-M,149500.00
Daniel,Chac√≥n,Colorado Rapids,D,67360.00
Tom√°s,Chancalay,New England Revolution,M,666250.00
Maikel,Chang,Real Salt Lake,M,170000.00
Diego,Char√°,Portland Timbers,M,650000.00
Yimmi,Char√°,Portland Timbers,M,1500000.00
Yevgen,Cheberko,Columbus Crew,D,370000.00
Giorgio,Chiellini,LAFC,D,1000000.00
Mathieu,Choini√®re,CF Montreal,M,250000.00
Machop,Chol,Atlanta United,M,89716.00
Cam,Cilley,San Jose Earthquakes,M,67360.00
Ozzie,Cisneros,Sporting Kansas City,M-F,85444.00
Abdoulaye,Cissoko,Seattle Sounders FC,D,85444.00
Steve,Clark,Houston Dynamo,GK,560000.00
Stefan,Cleveland,Seattle Sounders FC,GK,200000.00
Noah,Cobb,Atlanta United,D,67360.00
Alonso,Coello,Toronto FC,D-M,67360.00
Jackson,Conway,Atlanta United,F,89716.00
Enzo,Copetti,Charlotte FC,F,1000000.00
Gabriele,Corbo,CF Montreal,D,216350.00
Gabriel,Cordeiro Pirani,DC United,M,530000.00
Carlos,Coronel,New York Red Bulls,GK,490000.00
Guzm√°n,Corujo,Charlotte FC,D,625000.00
Douglas,Costa,LA Galaxy,M,3000000.00
S√©ga,Coulibaly,Major League Soccer,D,480000.00
Cade,Cowell,San Jose Earthquakes,M,600000.00
Maxime,Cr√©peau,LAFC,GK,350000.00
Brandan,Craig,Philadelphia Union,D,115000.00
Michael,Creek,St. Louis City SC,GK,85444.00
Benja,Cremaschi,Inter Miami,M,67360.00
Danny,Crisostomo,LAFC,D-M,85444.00
Andr√©s,Cubas,Vancouver Whitecaps,D-M,1050000.00
Mauricio,Cuevas,LA Galaxy,D,100000.00
Braian,Cufr√©,New York City FC,D,672000.00
Ant√¥nio Carlos,Cunha Capocasali Junior,Orlando City SC,D,900000.00
Rafael,Czichos,Chicago Fire,D,1300000.00
Evander,da Silva Ferreira,Portland Timbers,M,1885000.00
Luis,D√≠az,Colorado Rapids,M,500000.00
Cristian,D√°jome,DC United,M,900000.00
Bajung,Darboe,LAFC,M,67360.00
√âric,Davis,DC United,D,180000.00
Sean,Davis,Nashville SC,M,825000.00
Jake,Davis,Sporting Kansas City,D,85444.00
Ocimar,de Almeida J√∫nior,Orlando City SC,M,360000.00
Thiago Eduardo,de Andrade,New York City FC,M,200000.00
Geovane,de Jesus Rocha,FC Dallas,D,250000.00
Jos√© Artur,de Lima J√∫nior,Houston Dynamo,D-M,600000.00
Gregore,de Magalh√£es da Silva,Inter Miami,D-M,725000.00
Daniel,de Sousa Britto,San Jose Earthquakes,GK,400000.00
Jonathan,Dean,Chicago Fire,D,85444.00
Brecht,Dejaegere,Charlotte FC,M,820000.00
Mark,Delgado,LA Galaxy,M,750000.00
Nick,DePuy,Nashville SC,D,252000.00
Bakaye,Dibassy,Minnesota United,D,600000.00
Eric,Dick,Minnesota United,GK,85444.00
Adama,Diomande,Toronto FC,F,525000.00
Hamady,Diop,Charlotte FC,D,67360.00
Cl√©ment,Diop,Atlanta United,GK,85444.00
Sofiane,Djeffal,Austin FC,M,75325.00
Moussa,Djitt√©,Austin FC,F,600000.00
Ethan,Dobbelaere,Seattle Sounders FC,M,104000.00
Ronald,Donkor,New York Red Bulls,M,120000.00
Chris,Donovan,Philadelphia Union,F,68775.00
Griffin,Dorsey,Houston Dynamo,D,150000.00
CJ,dos Santos,Inter Miami,GK,67360.00
Micael,dos Santos Silva,Houston Dynamo,D,120000.00
Hassani,Dotson,Minnesota United,M,450000.00
Ousmane,Doumbia,Chicago Fire,D-M,420000.00
Sebasti√°n,Driussi,Austin FC,M-F,3800000.00
√ârik,Due√±as,LAFC,M,125000.00
Cameron,Duke,Sporting Kansas City,M,137500.00
Bryce,Duke,CF Montreal,M,150000.00
Cameron,Dunbar,Minnesota United,M,165000.00
Kyle,Duncan,New York Red Bulls,D,500000.00
Chris,Durkin,DC United,D-M,425000.00
Dom,Dwyer,Major League Soccer,F,85444.00
Jeremy,Ebobisse,San Jose Earthquakes,F,763980.00
Daniel,Edelman,New York Red Bulls,D-M,85444.00
Raheem,Edwards,LA Galaxy,D,315000.00
Michael,Edwards,Colorado Rapids,D,125000.00
Earl,Edwards Jr.,New England Revolution,GK,125000.00
Jack,Elliott,Philadelphia Union,D,750000.00
Fred,Emmings,Minnesota United,GK,85444.00
Herbert,Endeley,FC Dallas,D,67360.00
Emeka,Eneli,Real Salt Lake,D,67360.00
Ramiro,Enrique,Orlando City SC,F,318182.00
Franco,Escobar,Houston Dynamo,D,475000.00
Cristian,Espinoza,San Jose Earthquakes,M,1150000.00
R√≥ger,Espinoza,Sporting Kansas City,M,85444.00
Bento,Estrela,New York Red Bulls,M,85444.00
Derrick,Etienne Jr.,Atlanta United,M,676250.00
Diego,Fag√∫ndez,LA Galaxy,M,1000000.00
Jos√©,Fajardo,DC United,F,240000.00
Mamadou,Fall,LAFC,D,149000.00
Facundo,Far√≠as,Inter Miami,M-F,500000.00
Marco,Farf√°n,FC Dallas,D,425000.00
Zack,Farnsworth,Real Salt Lake,D,115000.00
Andrew,Farrell,New England Revolution,D,651250.00
Mohamed,Farsi,Columbus Crew,D,67360.00
Lucas Maciel,Felix,New England Revolution,M,85444.00
Marcus,Ferkranus,LA Galaxy,D,125000.00
Juli√°n,Fern√°ndez,New York City FC,M,540000.00
Omir,Fernandez,New York Red Bulls,M,161000.00
Jes√∫s,Ferreira,FC Dallas,F,1650000.00
Sebasti√°n,Ferreira,Houston Dynamo,F,1760000.00
Ethan,Finlay,Austin FC,M,425000.00
Leon,Flach,Philadelphia Union,M,280000.00
Kristian,Fletcher,DC United,M,85444.00
Danny,Flores,Sporting Kansas City,M,67360.00
CJ,Fodrey,Austin FC,M,67360.00
Andreu,Font√†s,Sporting Kansas City,D,450000.00
Kortne,Ford,Sporting Kansas City,D,165000.00
Ajani,Fortune,Atlanta United,M,67360.00
Isaiah,Foster,FC Cincinnati,D,85444.00
Franco,Fragapane,Minnesota United,M,260000.00
Iv√°n,Franco,Houston Dynamo,M,335000.00
Kobe,Franklin,Toronto FC,D,67360.00
Liam,Fraser,FC Dallas,D-M,130000.00
Ian,Fray,Inter Miami,D,110000.00
Alex,Freeman,Orlando City SC,D,67360.00
Tyler,Freeman,Nashville SC,F,67360.00
Matt,Freese,New York City FC,GK,161250.00
Stefan,Frei,Seattle Sounders FC,GK,650000.00
Diego,G√≥mez,Inter Miami,M,500000.00
Tom√°s,G√≥mez,Real Salt Lake,GK,85444.00
Carlos Andr√©s,G√≥mez,Real Salt Lake,M,220000.00
Yeimar,G√≥mez Andrade,Seattle Sounders FC,D,672732.00
Raymon,Gaddis,FC Cincinnati,D,300000.00
McKinze,Gaines,Charlotte FC,M,100000.00
Julian,Gaines,LAFC,D,104004.00
Jeff,Gal,Chicago Fire,GK,85444.00
Jon,Gallagher,Austin FC,D,300000.00
Pedro,Gallese,Orlando City SC,GK,720000.00
Braian,Galv√°n,Colorado Rapids,M,365700.00
Jeremy,Garay,DC United,D-M,67360.00
Mender,Garc√≠a,Minnesota United,F,350000.00
Justin,Garces,Atlanta United,GK,68775.00
Chase,Gasper,Houston Dynamo,D,400000.00
Ryan,Gauld,Vancouver Whitecaps,M-F,2400000.00
Luka,Gavran,Toronto FC,GK,67360.00
D√°niel,Gazdag,Philadelphia Union,M-F,1200000.00
Alex,Gersbach,Colorado Rapids,D,230000.00
Georgios,Giakoumakis,Atlanta United,F,1576636.00
Carles,Gil,New England Revolution,M-F,3250000.00
Nacho,Gil,New England Revolution,M,375000.00
Gast√≥n,Gim√©nez,Chicago Fire,D-M,1600000.00
Francisco,Ginella,LAFC,M,475000.00
Nicholas,Gioacchini,St. Louis City SC,F,380004.00
Justen,Glad,Real Salt Lake,D,725000.00
Jakob,Glesnes,Philadelphia Union,D,900000.00
Chris,Gloster,Major League Soccer,D,250000.00
Caden,Glover,St. Louis City SC,F,67360.00
An√≠bal,Godoy,Nashville SC,D-M,750000.00
Gast√≥n,Gonz√°lez,Orlando City SC,M,330000.00
Mario,Gonz√°lez,LAFC,F,888500.00
Omar,Gonz√°lez,New England Revolution,D,425000.00
Leandro,Gonz√°lez P√≠rez,Inter Miami,D,935004.00
Tayvon,Gray,New York City FC,D,300000.00
Jacob,Greene,DC United,D,89716.00
Ruan,Greg√≥rio Teixeira,DC United,D,400000.00
J√°n,Gregus,Minnesota United,M,240000.00
Julian,Gressel,Columbus Crew,M,884000.00
Adam,Grinwis,Orlando City SC,GK,85444.00
Carlos,Gruezo,San Jose Earthquakes,D-M,1350000.00
Cristi√°n,Guti√©rrez,Toronto FC,D,300000.00
Brian,Guti√©rrez,Chicago Fire,M-F,200004.00
Diego,Guti√©rrez,Portland Timbers,F,68775.00
Felipe,Guti√©rrez,Sporting Kansas City,M,165000.00
Andrew,Gutman,Colorado Rapids,D,350000.00
Brad,Guzan,Atlanta United,GK,600000.00
Justin,Haak,New York City FC,M,150000.00
Luke,Haakenson,Nashville SC,M,88200.00
Kamron,Habibullah,Vancouver Whitecaps,M,85444.00
Teenage,Hadebe,Houston Dynamo,D,1122900.00
Nick,Hagglund,FC Cincinnati,D,175000.00
Maren,Haile-Selassie,Chicago Fire,M,264000.00
Julian,Hall,New York Red Bulls,M,85444.00
Michael,Halliday,Orlando City SC,D,110000.00
Bret,Halsey,FC Cincinnati,D,85444.00
Ahmed,Hamdi,CF Montreal,M-F,500000.00
Ian,Harkes,New England Revolution,M,240000.00
Cameron,Harper,New York Red Bulls,M,250000.00
Nathan,Harriel,Philadelphia Union,D,160000.00
Calvin,Harris,Colorado Rapids,M,100000.00
Thomas,Hasal,Vancouver Whitecaps,GK,203000.00
Stuart,Hawkins,Seattle Sounders FC,D,67360.00
Matt,Hedges,Austin FC,D,776250.00
Chris,Hegardt,Charlotte FC,D-M,85444.00
Fabian,Herbers,Chicago Fire,M,310000.00
Cucho,Hern√°ndez,Columbus Crew,F,2600000.00
Ronald,Hern√°ndez,Atlanta United,D,375000.00
Felipe,Hern√°ndez,Sporting Kansas City,M,175000.00
Javier,Hern√°ndez Balc√°zar,LA Galaxy,F,6000000.00
Aar√≥n,Herrera,CF Montreal,D,725000.00
H√©ctor,Herrera,Houston Dynamo,M,4750000.00
Bode,Hidalgo,Real Salt Lake,D,104000.00
Kyle,Hiebert,St. Louis City SC,D,85444.00
Brendan,Hines-Ike,DC United,D,425000.00
Bongokuhle,Hlongwane,Minnesota United,M,560000.00
Junior,Hoilett,Vancouver Whitecaps,M,228000.00
Ryan,Hollingshead,LAFC,D,515000.00
Erik,Holt,Real Salt Lake,D,200000.00
Jackson,Hopkins,DC United,F,85444.00
Matthew,Hoppe,San Jose Earthquakes,F,650004.00
Keegan,Hughes,Columbus Crew,D,85444.00
Erik,Hurtado,DC United,F,85444.00
Franco,Ibarra,Atlanta United,D-M,550000.00
Sebastien,Ibeagha,FC Dallas,D,520000.00
Sunusi,Ibrahim,CF Montreal,F,200000.00
Tega,Ikoba,Portland Timbers,F,89716.00
Mitja,Ilenic,New York City FC,D,204000.00
Ilias,Iliadis,CF Montreal,D-M,85444.00
Marko,Ilic,Colorado Rapids,GK,420000.00
Asier,Illarramendi,FC Dallas,D-M,300000.00
Lorenzo,Insigne,Toronto FC,M,7500000.00
Clint,Irwin,Minnesota United,GK,190000.00
Aljaz,Ivacic,Portland Timbers,GK,375000.00
Emmanuel,Iwe,Minnesota United,M,67360.00
Kamil,J√≥zwiak,Charlotte FC,M,1100000.00
Ousman,Jabang,CF Montreal,D-M,67360.00
Jacob,Jackson,New England Revolution,GK,68775.00
Aziel,Jackson,St. Louis City SC,M-F,85444.00
Bertin,Jacquesson,Real Salt Lake,M,67360.00
Eldin,Jakupovic,LAFC,GK,100000.00
Robin,Jansson,Orlando City SC,D,575000.00
Andres,Jasson,New York City FC,M,125000.00
Mohanad,Jeahze,DC United,D,660000.00
Corentin,Jean,Inter Miami,F,720000.00
Cole,Jensen,Inter Miami,GK,67360.00
Isak,Jensen,St. Louis City SC,M,175000.00
Sang-bin,Jeong,Minnesota United,F,548000.00
Ryen,Jiba,Minnesota United,D,67360.00
Jes√∫s,Jim√©nez,FC Dallas,F,1100000.00
H√©ctor,Jim√©nez,Austin FC,D,85444.00
Jonathan,Jim√©nez,New York City FC,M,68775.00
Stiven,Jimenez,FC Cincinnati,D-M,67360.00
Sean,Johnson,Toronto FC,GK,405751.00
Levonte,Johnson,Vancouver Whitecaps,M,67360.00
Derrick,Jones,Charlotte FC,M,325000.00
DeJuan,Jones,New England Revolution,D,375000.00
Dejan,Joveljic,LA Galaxy,F,651250.00
Preston,Judd,LA Galaxy,F,67360.00
Anderson,Julio,Real Salt Lake,M,750000.00
Sam,Junqua,FC Dallas,D,85444.00
Kristijan,Kahlina,Charlotte FC,GK,500000.00
Brent,Kallman,Minnesota United,D,160000.00
Kei,Kamara,Chicago Fire,F,200000.00
Bernard,Kamungo,FC Dallas,M,85444.00
Alec,Kann,FC Cincinnati,GK,200000.00
Mark-Anthony,Kaye,New England Revolution,M,750000.00
Axel,Kei,Real Salt Lake,F,85444.00
Aboubacar,Keita,Colorado Rapids,D,215000.00
Kipp,Keller,Austin FC,D,100000.00
Deandre,Kerr,Toronto FC,M,85444.00
Henry,Kessler,New England Revolution,D,200000.00
Logan,Ketterer,CF Montreal,GK,88200.00
Benjamin,Kikanovic,San Jose Earthquakes,M,130000.00
Gadi,Kinda,Sporting Kansas City,M-F,850000.00
Sota,Kitahara,Seattle Sounders FC,D-M,67360.00
Jo√£o,Klauss de Mello,St. Louis City SC,F,1230000.00
Mateusz,Klich,DC United,M,1900000.00
Jonathan,Klinsmann,LA Galaxy,GK,200004.00
Zan,Kolmanic,Austin FC,D,325000.00
Amet,Korca,FC Dallas,D,85444.00
Georgios,Koutsias,Chicago Fire,F,470800.00
Sebastian,Kowalczyk,Houston Dynamo,M-F,85444.00
Filip,Krastev,LAFC,M,500000.00
Damir,Kreilach,Real Salt Lake,M,1650000.00
Sergiy,Kryvtsov,Inter Miami,D,555500.00
Theodore,Ku-DiPietro,DC United,M-F,85444.00
Yuya,Kubo,FC Cincinnati,M-F,1091000.00
Jojea,Kwizera,CF Montreal,M,68775.00
Jasper,L√∂ffelsend,Real Salt Lake,M,96600.00
Eduard,L√∂wen,St. Louis City SC,M,1200000.00
Math√≠as,Laborda,Vancouver Whitecaps,D,760000.00
Jake,LaCava,Inter Miami,M,85444.00
Kevon,Lambert,Real Salt Lake,D-M,180000.00
Lassi,Lappalainen,CF Montreal,M,630000.00
Oliver,Larraz,Colorado Rapids,M,85444.00
Richie,Laryea,Vancouver Whitecaps,D,1436338.00
Damian,Las,Austin FC,GK,67360.00
Ariel,Lassiter,CF Montreal,M,220000.00
Randall,Leal,Nashville SC,M,1000000.00
Richard,Ledezma,New York City FC,M-F,350000.00
Kelvin,Leerdam,LA Galaxy,D,300000.00
Tim,Leibold,Sporting Kansas City,D,500000.00
Brooks,Lennon,Atlanta United,D,700000.00
Tony,Leone,LAFC,D,125000.00
Jonathan,Lewis,Colorado Rapids,M,410000.00
Danny,Leyva,Colorado Rapids,D-M,200000.00
Nick,Lima,Austin FC,D,375000.00
Lucas,Lima Linhares,New York Red Bulls,M-F,1155000.00
Jaylin,Lindsey,Charlotte FC,D,250000.00
Sebastian,Lletget,FC Dallas,M,700000.00
Saba,Lobjanidze,Atlanta United,M,2052000.00
Robin,Lod,Minnesota United,M,900000.00
Nicol√°s,Lodeiro,Seattle Sounders FC,M-F,2640000.00
Aaron,Long,LAFC,D,1199772.00
Ahmed,Longmire,Nashville SC,D,67360.00
Marvin,Lor√≠a,Portland Timbers,M,200000.00
Evan,Louro,FC Cincinnati,GK,88200.00
Daniel,Lovitz,Nashville SC,D,551250.00
Damion,Lowe,Philadelphia Union,D,275000.00
Favian,Loyola,Orlando City SC,M-F,67360.00
Rafael,Lucas Cardoso dos Santos,Orlando City SC,D,300000.00
Diego,Luna,Real Salt Lake,M-F,120000.00
Adam,Lundqvist,Austin FC,D,350000.00
Ben,Lundt,St. Louis City SC,GK,100000.00
Jack,Lynn,Orlando City SC,F,68775.00
Larrys,Mabiala,Portland Timbers,D,400000.00
Aim√©,Mabika,Toronto FC,D,85444.00
Zac,MacMath,Real Salt Lake,GK,350000.00
Lukas,MacNaughton,Nashville SC,D,85444.00
Jack,Maher,Nashville SC,D,260004.00
Cassius,Mailula,Toronto FC,M,275000.00
Olwethu,Makhanya,Philadelphia Union,D,150000.00
Christian,Makoun,New England Revolution,D,225000.00
Adilson,Malanda,Charlotte FC,D,275000.00
Denil,Maldonado,LAFC,D,355000.00
Elias,Manoel Alves de Paula,New York Red Bulls,F,420000.00
JT,Marcinkowski,San Jose Earthquakes,GK,350000.00
AJ,Marcucci,New York Red Bulls,GK,85444.00
Paul,Marie,San Jose Earthquakes,D,180000.00
Anthony,Markanich,St. Louis City SC,D,68775.00
George,Marks,Charlotte FC,GK,68775.00
Mikael,Marqu√©s,Minnesota United,D,100000.00
Jahkeele,Marshall-Rutty,Toronto FC,D,152500.00
Nick,Marsman,Major League Soccer,GK,474996.00
Josef,Mart√≠nez,Inter Miami,F,4000000.00
Jos√©,Mart√≠nez,Philadelphia Union,D-M,550000.00
Jos√© Antonio,Mart√≠nez,FC Dallas,D,700000.00
Alonso,Mart√≠nez,New York City FC,M,249996.00
Ben,Martino,Nashville SC,GK,67360.00
Felipe,Martins,Orlando City SC,M,85444.00
Lu√≠s,Martins,Vancouver Whitecaps,D,300000.00
Thiago,Martins,New York City FC,D,1650000.00
Alexandru,Matan,Columbus Crew,M,450000.00
Jimmy,Maurer,FC Dallas,GK,300000.00
Chris,Mavinga,LA Galaxy,D,399996.00
Andreas,Maxs√∂,Colorado Rapids,D,1100000.00
Olivier,Mbaizo,Philadelphia Union,D,300000.00
Hugo,Mbongue,Toronto FC,F,67360.00
John,McCarthy,LAFC,GK,115000.00
Dax,McCarty,Nashville SC,M,275000.00
Aiden,McFadden,Atlanta United,D,85444.00
Christian,McFarlane,New York City FC,D,85444.00
Jack,McGlynn,Philadelphia Union,M,160000.00
Zac,McGraw,Portland Timbers,D,130000.00
Duncan,McGuire,Orlando City SC,F,67360.00
Kendall,McIntosh,Sporting Kansas City,GK,130000.00
Tommy,McNamara,New England Revolution,M,420000.00
Christopher,McVey,Inter Miami,D,219996.00
Ryan,Meara,New York Red Bulls,GK,280000.00
Cruz,Medina,San Jose Earthquakes,M-F,85444.00
Jimmy,Medranda,Columbus Crew,D,300000.00
Tim,Melia,Sporting Kansas City,GK,625000.00
Jonathan,Mensah,San Jose Earthquakes,D,1025000.00
Justin,Meram,Charlotte FC,M,450000.00
Lionel,Messi,Inter Miami,M,12000000.00
Matt,Miazga,FC Cincinnati,D,1500000.00
Novak,Micovic,LA Galaxy,GK,67360.00
Matko,Miljevic,CF Montreal,M,545455.00
Kamal,Miller,Inter Miami,D,375000.00
Eric,Miller,Portland Timbers,D,120000.00
Tyler,Miller,DC United,GK,400000.00
Juan,Mina,New York Red Bulls,D,131000.00
Jo√£o Paulo,Mior,Seattle Sounders FC,M,1300000.00
Cody,Mizell,New York City FC,GK,85444.00
Shak,Mohammed,Orlando City SC,M,85444.00
Kevin,Molino,Columbus Crew,M,735367.00
Alex,Monis,Chicago Fire,M,85444.00
Jamiro,Monteiro,San Jose Earthquakes,M,1200000.00
Fredy,Montero,Seattle Sounders FC,F,195000.00
Shaq,Moore,Nashville SC,D,725000.00
Felipe,Mora,Portland Timbers,F,120000.00
Santiago,Morales,Inter Miami,M-F,67360.00
Efra√≠n,Morales,Atlanta United,D,85444.00
Alfredo,Morales,New York City FC,M,600000.00
Maxi,Moralez,New York City FC,M-F,480000.00
Steven,Moreira,Columbus Crew,D,700000.00
Santiago,Moreno,Portland Timbers,M,340000.00
J√∫nior,Moreno,FC Cincinnati,D-M,349992.00
Lewis,Morgan,New York Red Bulls,M,1200000.00
Aidan,Morris,Columbus Crew,M,500000.00
Jake,Morris,Columbus Crew,D,85444.00
Jordan,Morris,Seattle Sounders FC,M,1550000.00
Juan David,Mosquera,Portland Timbers,D,184000.00
Yerson,Mosquera,FC Cincinnati,D,600000.00
Edwin,Mosquera,Atlanta United,M,375000.00
Chris,Mueller,Chicago Fire,M,600000.00
Hany,Mukhtar,Nashville SC,M-F,2900000.00
Jos√©,Mulato,FC Dallas,F,85444.00
Daniel,Munie,San Jose Earthquakes,D,67360.00
Mujeeb,Murana,Houston Dynamo,D,67360.00
Jes√∫s,Murillo,LAFC,D,450000.00
Ian,Murphy,FC Cincinnati,D,72050.00
Danny,Musovski,Real Salt Lake,F,98687.00
Alex,Muyl,Nashville SC,M,500000.00
Tristan,Muyumba,Atlanta United,D-M,450000.00
Andy,N√°jar,DC United,D,450000.00
Darlington,Nagbe,Columbus Crew,M,1800000.00
Federico,Navarro,Chicago Fire,D-M,450000.00
Miguel,Navarro,Chicago Fire,D,260004.00
Rafael,Navarro Leal,Colorado Rapids,F,1100000.00
Hassan,Ndam,New York Red Bulls,D,88200.00
Logan,Ndenbe,Sporting Kansas City,D,350000.00
Jalen,Neal,LA Galaxy,D,125000.00
Dylan,Nealis,New York Red Bulls,D,225000.00
Sean,Nealis,New York Red Bulls,D,400000.00
Jack,Neeley,Charlotte FC,D,67360.00
Franco,Negri,Inter Miami,D,225000.00
Michael,Nelson,Houston Dynamo,GK,135000.00
John,Nelson,St. Louis City SC,D,160000.00
Jake,Nerwinski,St. Louis City SC,D,250000.00
Harvey,Neville,Inter Miami,D,67360.00
J.C.,Ngando,Vancouver Whitecaps,D-M,67360.00
Serge,Ngoma,New York Red Bulls,M,67360.00
Sam,Nicholson,Colorado Rapids,M,325000.00
Jaroslaw,Niezgoda,Portland Timbers,F,850000.00
Joakim,Nilsson,St. Louis City SC,D,1102450.00
Matthew,Nocita,New York Red Bulls,D,88200.00
Nolan,Norris,FC Dallas,D,67360.00
Tsiki,Ntsabeleng,FC Dallas,M-F,85444.00
Obinna,Nwobodo,FC Cincinnati,D-M,1155000.00
Moses,Nyeman,Real Salt Lake,D-M,115000.00
Owen,O'Malley,St. Louis City SC,D,67360.00
Shane,O'Neill,Toronto FC,D,350000.00
Kevin,O'Toole,New York City FC,D,85444.00
J√°der,Obrian,FC Dallas,M,382500.00
Alfonso,Ocampo-Ch√°vez,Austin FC,F,67360.00
David,Ochoa,Inter Miami,D-M,67360.00
Emmanuel,Ochoa,San Jose Earthquakes,GK,130000.00
Richard,Odada,Philadelphia Union,M,95000.00
Chinonso,Offor,CF Montreal,F,200000.00
Curtis,Ofori,New York Red Bulls,D,67360.00
Mart√≠n,Ojeda,Orlando City SC,M,840000.00
Braian,Ojeda,Real Salt Lake,M,590000.00
Jean Mota,Oliveira de Sousa,Inter Miami,M,750000.00
Cristian,Olivera,LAFC,M,650000.00
Tani,Oluwaseyi,Minnesota United,F,68775.00
Wyatt,Omsberg,Chicago Fire,D,170000.00
Kwadwo,Opoku,CF Montreal,M,310344.00
Arqu√≠mides,Ord√≥√±ez,FC Cincinnati,F,85444.00
Nathan,Ordaz,LAFC,F,85444.00
Sergio,Oregel,Chicago Fire,M,67360.00
Jaziel,Orozco,Real Salt Lake,D,85444.00
Jonathan,Osorio,Toronto FC,M,1400000.00
Tom√°s,Ostr√°k,St. Louis City SC,M-F,700000.00
Javier,Otero,Orlando City SC,GK,67360.00
Bryan,Oviedo,Real Salt Lake,D,325000.00
Prince Osei,Owusu,Toronto FC,F,600000.00
Samuel,Owusu,New York City FC,D,67360.00
Jonathan,P√©rez,LA Galaxy,M,250000.00
Devin,Padelford,Minnesota United,D,67360.00
Maarten,Paes,FC Dallas,GK,300000.00
Nelson,Palacio,Real Salt Lake,M,300000.00
Diego,Palacios,LAFC,D,540000.00
Sergi,Palencia,LAFC,D,225000.00
Jack,Panayotou,New England Revolution,M-F,67360.00
Elliot,Panicco,Nashville SC,GK,89716.00
James,Pantemis,CF Montreal,GK,144996.00
Cristhian,Paredes,Portland Timbers,M,550000.00
Isaiah,Parente,Columbus Crew,M,85444.00
Isaiah,Parker,FC Dallas,D,67360.00
Tim,Parker,St. Louis City SC,D,1000000.00
Keaton,Parks,New York City FC,D-M,550000.00
Ilijah,Paul,Real Salt Lake,F,67360.00
Nathan Raphael,Pelae Cardoso,San Jose Earthquakes,D,700000.00
Mat√≠as,Pellegrini,New York City FC,M,1025000.00
Andr√©s,Perea,New York City FC,D-M,525000.00
Daniel,Pereira,Austin FC,M,100000.00
Mauricio,Pereyra,Orlando City SC,M-F,675000.00
Miguel,Perez,St. Louis City SC,M,67360.00
Jordan,Perruzza,Toronto FC,F,130000.00
Nebiyou,Perry,Nashville SC,M,67360.00
Luca,Petrasso,Orlando City SC,D,85444.00
Raoul,Petretta,Toronto FC,D,625000.00
Faf√†,Picault,Nashville SC,M,650000.00
Selmir,Pidro,St. Louis City SC,D,325000.00
Nelson,Pierre,Philadelphia Union,F,67360.00
Delentz,Pierre,Real Salt Lake,D,67360.00
Kayden,Pierre,Sporting Kansas City,D,85444.00
Samuel,Piette,CF Montreal,D-M,397375.00
Mauricio,Pineda,Chicago Fire,D,255000.00
Donovan,Pines,DC United,D,200000.00
Malik,Pinto,FC Cincinnati,D-M,67360.00
Matt,Polster,New England Revolution,D-M,500000.00
C√©lio Antonio,Pompeu,St. Louis City SC,M,68775.00
Paxton,Pomykal,FC Dallas,M-F,800000.00
Alvas,Powell,FC Cincinnati,D,190000.00
Jack,Price,Colorado Rapids,M,762500.00
Ralph,Priso-Mbongue,Colorado Rapids,D-M,89716.00
Andrew,Privett,Charlotte FC,D,67360.00
Kacper,Przybylko,Chicago Fire,F,1100004.00
Riqui,Puig,LA Galaxy,M,1650000.00
Teemu,Pukki,Minnesota United,F,3200000.00
Alan,Pulido,Sporting Kansas City,F,2200000.00
John,Pulskamp,Sporting Kansas City,GK,150000.00
Juan Jos√©,Purata,Atlanta United,D,400000.00
Nelson,Qui√±√≥nes,Houston Dynamo,M,216996.00
Facundo,Quign√≥n,FC Dallas,D-M,836000.00
Philip,Quinton,Columbus Crew,D,67360.00
Romell,Quioto,CF Montreal,F,950000.00
Daniel,R√≠os,Houston Dynamo,M-F,85444.00
Nemanja,Radoja,Sporting Kansas City,D-M,800000.00
Jeremy,Rafanello,Philadelphia Union,M-F,85444.00
Jackson,Ragen,Seattle Sounders FC,D,85444.00
Brooklyn,Raines,Houston Dynamo,M,85444.00
Christian,Ramirez,Columbus Crew,F,400000.00
Greg,Ranjitsingh,Toronto FC,GK,95000.00
Ryan,Raposo,Vancouver Whitecaps,M,120000.00
Justin,Rasmussen,Portland Timbers,D,85444.00
Sean,Rea,CF Montreal,M-F,85444.00
Matt,Real,Philadelphia Union,D,140000.00
Rodney,Redes,Austin FC,M,440000.00
Jayden,Reid,New York Red Bulls,D,67360.00
Justin,Rennicks,New England Revolution,F,125000.00
Ben,Reveno,New England Revolution,D,68775.00
Andr√©s,Reyes,New York Red Bulls,D,450000.00
Justin,Reynolds,Chicago Fire,D,67360.00
Emanuel,Reynoso,Minnesota United,M-F,1600000.00
Spencer,Richey,Chicago Fire,GK,132000.00
Will,Richmond,San Jose Earthquakes,M,68775.00
Emiliano,Rigoni,Austin FC,M,1599996.00
Chris,Rindov,Sporting Kansas City,D,67360.00
Alexander,Ring,Austin FC,M,1500000.00
Birk,Risa,New York City FC,D,600000.00
Wilfredo,Rivera,Orlando City SC,F,85444.00
Luis,Rivera,Real Salt Lake,D,67360.00
Dami√°n,Rivera,New England Revolution,M,88200.00
Nigel,Robertha,DC United,F,550000.00
Robbie,Robinson,Inter Miami,M,200000.00
Miles,Robinson,Atlanta United,D,1400000.00
Harrison,Robledo,FC Cincinnati,D-M,85444.00
Emerson,Rodr√≠guez,Inter Miami,M,330000.00
Missael,Rodr√≠guez,Chicago Fire,F,67360.00
Mart√≠n,Rodr√≠guez,DC United,M,1000000.00
Memo,Rodr√≠guez,Austin FC,M,180000.00
Santiago,Rodr√≠guez,New York City FC,M-F,1200000.00
Ant√¥nio Josenildo,Rodrigues de Oliveira,San Jose Earthquakes,D,600000.00
Allan,Rodriguez,Chicago Fire,D-M,85444.00
Abraham,Rodriguez,Colorado Rapids,GK,88200.00
Cristian,Rold√°n,Seattle Sounders FC,M,1320000.00
√Ålex,Rold√°n,Seattle Sounders FC,D,300000.00
Jhohan,Roma√±a,Austin FC,D,460000.00
Brian,Romero,Charlotte FC,M,67360.00
Tom√°s,Romero,Toronto FC,GK,120000.00
Abraham,Romero,LAFC,GK,85444.00
Dave,Romney,New England Revolution,D,625000.00
Connor,Ronan,Colorado Rapids,M,460000.00
Diego,Rosales,LAFC,D,67360.00
Joseph,Rosales,Minnesota United,M,85444.00
Uri,Rosell,LA Galaxy,D-M,500000.00
Keegan,Rosenberry,Colorado Rapids,D,400000.00
Dany,Rosero,Sporting Kansas City,D,456000.00
Matheus,Rossetto,Atlanta United,M,650000.00
Diego,Rossi,Columbus Crew,M,2625000.00
Sigurd,Rosted,Toronto FC,D,763636.00
Paul,Rothrock,Seattle Sounders FC,F,85444.00
Kelyn,Rowe,Seattle Sounders FC,M,300000.00
Rubio,Rub√≠n,Real Salt Lake,F,600000.00
Diego,Rubio,Colorado Rapids,F,660000.00
Ra√∫l,Ruid√≠az,Seattle Sounders FC,F,2472000.00
Pablo,Ruiz,Real Salt Lake,M,500000.00
Albert,Rusn√°k,Seattle Sounders FC,M-F,1800000.00
Johnny,Russell,Sporting Kansas City,M,1000000.00
Jacen,Russell-Rowe,Columbus Crew,F,67360.00
Ilie,S√°nchez,LAFC,D-M,1207500.00
Ryan,Sailor,Inter Miami,D,85444.00
Adam,Salda√±a,LA Galaxy,M,93989.00
Nathan-Dylan,Saliba,CF Montreal,M,67360.00
Abdi,Salim,Orlando City SC,D,67360.00
D√°niel,Sall√≥i,Sporting Kansas City,M,1100000.00
Gaoussou,Samak√©,DC United,D,85444.00
Will,Sands,Columbus Crew,D,85444.00
James,Sands,New York City FC,D,1000000.00
Pedro,Santos,DC United,M,350000.00
Sergio,Santos,FC Cincinnati,F,700000.00
C.J.,Sapong,Toronto FC,F,600000.00
Hayden,Sargis,DC United,D,85444.00
Jefferson,Savarino,Real Salt Lake,M,1525000.00
Nick,Scardina,Charlotte FC,M,67360.00
Alessandro,Sch√∂pf,Vancouver Whitecaps,M-F,900000.00
Rodrigo,Schlegel,Orlando City SC,D,300000.00
Tate,Schmitt,Houston Dynamo,D,85444.00
Max,Schneider,St. Louis City SC,M,68775.00
Patrick,Schulte,Columbus Crew,GK,88200.00
Brady,Scott,Columbus Crew,GK,88200.00
Tarik,Scott,FC Dallas,M,67360.00
Dante,Sealy,FC Dallas,M,250000.00
Gabriel,Segal,New York City FC,F,67360.00
Eddie,Segura,LAFC,D,450000.00
Amar,Sejdic,Atlanta United,D-M,85444.00
Brandon,Servania,Toronto FC,M,475000.00
Jacob,Shaffelburg,Nashville SC,M,150000.00
Xherdan,Shaqiri,Chicago Fire,M-F,7350000.00
Billy,Sharp,LA Galaxy,F,240000.00
Khiry,Shelton,Sporting Kansas City,F,575000.00
Jonathan,Shore,New York City FC,M,67360.00
Marcelo,Silva,Real Salt Lake,D,600000.00
Xande,Silva,Atlanta United,M,480000.00
Judson,Silva Tavares,San Jose Earthquakes,D-M,250000.00
Vinicius,Silveira de Mello,Charlotte FC,F,315000.00
Luke,Singh,Toronto FC,D,85444.00
Jonathan,Sirois,CF Montreal,GK,85444.00
Pablo,Sisniega,Charlotte FC,GK,190000.00
Jack,Skahan,San Jose Earthquakes,M,88200.00
Joey,Skinner,Nashville SC,D,67360.00
Kyle,Smith,Orlando City SC,D,250000.00
Collin,Smith,FC Dallas,D,130000.00
Brad,Smith,Houston Dynamo,D,375000.00
Jan,Sobocinski,Charlotte FC,D,325000.00
Anton,Sorenson,Philadelphia Union,D,95000.00
Santiago,Sosa,Atlanta United,D-M,575000.00
Arnaud,Souquet,Chicago Fire,D,600000.00
Ryan,Spaulding,New England Revolution,D,85444.00
Steven,Sserwadda,New York Red Bulls,M,68775.00
Dayne,St. Clair,Minnesota United,GK,410000.00
Mason,Stajduhar,Orlando City SC,GK,130000.00
Nicol√°s,Stefanelli,Inter Miami,F,400000.00
Daniel,Steres,Houston Dynamo,D,355000.00
Peter,Stroud,New York Red Bulls,M,85444.00
Jared,Stroud,St. Louis City SC,M,88200.00
Brad,Stuver,Austin FC,GK,408000.00
Santiago,Su√°rez,New England Revolution,D,67360.00
Quinn,Sullivan,Philadelphia Union,M,120000.00
Hunter,Sulte,Portland Timbers,GK,85444.00
Lawson,Sunderland,Inter Miami,M,67360.00
Sam,Surridge,Nashville SC,F,2500000.00
Erik,Sviatchenko,Houston Dynamo,D,410532.00
Ben,Sweat,New England Revolution,D,250000.00
Karol,Swiderski,Charlotte FC,F,2200000.00
Ismael,Tajouri-Shradi,Minnesota United,M,85444.00
Yohei,Takaoka,Vancouver Whitecaps,GK,179520.00
Miguel,Tapias,Minnesota United,D,500012.00
Andrew,Tarbell,Houston Dynamo,GK,250000.00
Sidnei,Tavares,Colorado Rapids,M,525000.00
Robert,Taylor,Inter Miami,M,276000.00
D.J.,Taylor,Minnesota United,D,98261.00
Russell,Teibert,Vancouver Whitecaps,D-M,400000.00
Carlos,Ter√°n,Chicago Fire,D,260004.00
Dylan,Teves,Seattle Sounders FC,M,68775.00
Dagur Dan,Th√≥rhallsson,Orlando City SC,M,160000.00
N√∂kkvi,Th√≥risson,St. Louis City SC,M,150000.00
Jamal,Thiar√©,Atlanta United,F,977679.00
Andrew,Thomas,Seattle Sounders FC,GK,125000.00
Erik,Thommy,Sporting Kansas City,M-F,1550000.00
Kosi,Thompson,Toronto FC,M,85444.00
Tommy,Thompson,San Jose Earthquakes,D,115000.00
R√≥bert Orri,Thorkelsson,CF Montreal,D,175000.00
Timothy,Tillman,LAFC,M,500000.00
Keegan,Tingey,San Jose Earthquakes,D,67360.00
John,Tolkin,New York Red Bulls,D,400000.00
Nouhou,Tolo,Seattle Sounders FC,D,550000.00
Facundo,Torres,Orlando City SC,M,640000.00
Jairo,Torres,Chicago Fire,M,1100004.00
Joaqu√≠n,Torres,Philadelphia Union,M,260000.00
Christian,Torres,LAFC,M,112500.00
Dantouma,Toure,Major League Soccer,M,85444.00
Mason,Toye,CF Montreal,F,550000.00
Mohamed,Traore,LAFC,D,125000.00
Wil,Trapp,Minnesota United,D-M,850000.00
Miguel,Trauco,San Jose Earthquakes,D,500000.00
Jackson,Travis,Colorado Rapids,D,67360.00
Holden,Trent,Philadelphia Union,GK,67360.00
Niko,Tsakiris,San Jose Earthquakes,M,85444.00
Bill,Tuiloma,Charlotte FC,D,435000.00
Stephen,Turnbull,New York City FC,D,85444.00
Ema,Twumasi,FC Dallas,D,320000.00
Marinos,Tzionis,Sporting Kansas City,M,625000.00
Mikael,Uhre,Philadelphia Union,F,1700000.00
Nathan,Uiliam Foga√ßa,Portland Timbers,F,68775.00
V√≠ctor,Ulloa,Inter Miami,D-M,250000.00
Jere,Uronen,Charlotte FC,D,450000.00
Maximiliano,Urruti,Austin FC,F,700000.00
V√≠ctor,V√°zquez,Toronto FC,M-F,268000.00
Leo,V√§is√§nen,Austin FC,D,600000.00
Tomas,Vaclik,New England Revolution,GK,400000.00
Xavier,Valdez,Houston Dynamo,GK,85444.00
Felipe,Valencia,Inter Miami,M,85444.00
Jhojan,Valencia,Austin FC,D-M,400000.00
Zarek,Valentin,Minnesota United,D,85444.00
Gerardo,Valenzuela,FC Cincinnati,M-F,67360.00
Gustavo,Vallecilla,Columbus Crew,D,336000.00
Dante,Vanzeir,New York Red Bulls,F,1320000.00
Kerwin,Vargas,Charlotte FC,F,375000.00
Obed,Vargas,Seattle Sounders FC,M,67360.00
Indiana,Vassilev,St. Louis City SC,M-F,250000.00
Brandon,Vazquez,FC Cincinnati,F,900000.00
Carlos,Vela,LAFC,M,3000000.00
Alan,Velasco,FC Dallas,M,1200000.00
Brayan,Vera,Real Salt Lake,D,450000.00
Mat√≠as,Vera,Houston Dynamo,D-M,550000.00
Oscar,Verhoeven,San Jose Earthquakes,D,67360.00
Kenneth,Vermeer,Major League Soccer,GK,387334.00
Ranko,Veselinovic,Vancouver Whitecaps,D,495000.00
Jules-Anthony,Vilsaint,CF Montreal,F,85444.00
Pedro,Vite,Vancouver Whitecaps,M-F,592000.00
Gino,Vivi,LA Galaxy,M,67360.00
Robert,Voloder,Sporting Kansas City,D,425000.00
Giacomo,Vrioni,New England Revolution,F,1600000.00
Kai,Wagner,Philadelphia Union,D,630000.00
Casey,Walls,San Jose Earthquakes,D,150000.00
R√©mi,Walter,Sporting Kansas City,M,850000.00
Paul,Walters,FC Cincinnati,GK,67360.00
Victor,Wanyama,CF Montreal,D-M,1440000.00
Collen,Warner,Major League Soccer,M,85444.00
Taylor,Washington,Nashville SC,D,98688.00
Joel,Waterman,CF Montreal,D,161369.00
Akil,Watts,St. Louis City SC,D-M,67360.00
Patrick,Weah,Minnesota United,M,85444.00
Jude,Wellings,Real Salt Lake,M,85444.00
Quentin,Westberg,Atlanta United,GK,200000.00
Ashley,Westwood,Charlotte FC,D-M,750000.00
Brian,White,Vancouver Whitecaps,F,511000.00
Caleb,Wiley,Atlanta United,D,85444.00
Thomas,Williams,Orlando City SC,D,85444.00
Josh,Williams,Columbus Crew,D,325000.00
Derrick,Williams,DC United,D,800000.00
Eryk,Williamson,Portland Timbers,M,625000.00
Joe,Willis,Nashville SC,GK,375000.00
Danny,Wilson,Colorado Rapids,D,330000.00
Tyler,Wolff,Atlanta United,M,120000.00
Owen,Wolff,Austin FC,M,100000.00
Bobby,Wood,New England Revolution,F,400000.00
Laurence,Wyke,Nashville SC,D,85444.00
Karifa,Yao,Vancouver Whitecaps,D,89716.00
Darren,Yapi,Colorado Rapids,F,85444.00
William,Yarbrough,Colorado Rapids,GK,400000.00
Joshua,Yaro,St. Louis City SC,D,85444.00
Dru,Yearwood,New York Red Bulls,M,525000.00
Yaw,Yeboah,Columbus Crew,M,600000.00
DeAndre,Yedlin,Inter Miami,D,825000.00
Maya,Yoshida,LA Galaxy,D,800000.00
Jackson,Yueill,San Jose Earthquakes,M,750000.00
Luis,Zamudio,DC United,GK,85444.00
Gyasi,Zardes,Austin FC,F,800000.00
Eriq,Zavaleta,LA Galaxy,D,85444.00
Sean,Zawadzki,Columbus Crew,M,88200.00
Adri√°n,Zendejas,Charlotte FC,GK,85444.00
Walker,Zimmerman,Nashville SC,D,1800000.00
Rida,Zouhir,CF Montreal,M,85444.00
Ethan,Zubak,Nashville SC,F,195000.00
Dario,Zuparic,Portland Timbers,D,735000.00
Graham,Zusi,Sporting Kansas City,D,325000.00";

// Define the main function that returns a Result type.
// accepts a filter i.e. 5.0 type f64 and returns a DataFrame
// If everything is Ok, it returns `()`, otherwise it returns a `PolarsError`.
pub fn calculate(filter: f64) -> Result<DataFrame, PolarsError> {
    // Create a Cursor object from the SALARIES_DATA constant
    let file = Cursor::new(SALARIES_DATA);
    // columns first_name,last_name,team,position,salary
    // Read the CSV data using CsvReader
    let df = CsvReader::new(file)
        .has_header(true)
        .finish()?
        .lazy()
        .filter(col("salary").gt(lit(filter)))
        .groupby(vec![col("team")])
        .agg(&[
            col("position").count()])
        .collect()?;

    Ok(df)
}
