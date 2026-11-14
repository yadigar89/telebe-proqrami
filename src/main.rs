mod telebe;
use telebe::{Telebe, fayildan_oxu, fayla_yaz, oxu, tap};



fn main() {

    let mut telebeler = fayildan_oxu();

    loop{
        println!("\n1 - Telebelerin siyahisi:");
        println!("2 - Telebe elave et");
        println!("3 - Fayla saxla");
        println!("4 - Telebe axtar");
        println!("5 - Telebe sil");
        println!("6 - Cixis");

        let secim = oxu("Secim edin:");

        match secim.as_str(){
            "1" =>{
                for t in &telebeler{
                    println!("{}, {}, {}.", t.ad, t.yas, t.ixtisas);
                }
            }
            "2" => {
                let ad = oxu("Ad");
                let yas_metni = oxu("Yas");
                let ixtisas = oxu("Ixtisas");

                match yas_metni.parse::<i32>(){
                    Ok(yas) =>{
                        telebeler.push(Telebe { ad, yas, ixtisas });
                        println!("Elave edildi.");
                    }
                    Err(_) =>println!("Yas ededi duz deyil"),
                }
            }
            "3" =>{
                match fayla_yaz(&telebeler){
                    Ok(_) =>println!("Saxlanlidi"),
                    Err(e) =>println!("Xeta: {e}"),
                }
            }
            "4" =>{
                let ad = oxu("Axtarilan ad:");

                match tap(&telebeler, &ad) {
                    Some(t) =>println!("Tapildi: {}, {}, {}", t.ad, t.yas, t.ixtisas),
                    None =>println!("Bele telebe yoxdur"),
                }
            }
            "5" =>{

                let ad = oxu("Silinecek ad:");

                match telebeler.iter().position(|t| t.ad == ad){
                    Some(yer) =>{
                        telebeler.remove(yer);
                        println!("Silindi");
                    }
                    None =>println!("Bele telebe yoxdur.")
                }
            }
            "6" =>{
                println!("Gorusenedek!");
                break;
            }
            _ =>println!("Bele secim yoxdur."),
        }
    }
}