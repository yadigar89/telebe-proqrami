mod telebe;
use telebe::{Telebe, Tanit, fayildan_oxu, fayla_yaz, oxu, tap, tap_deyis};
enum Secim {
    Siyahi,
    Elave,
    Saxla,
    Axtar,
    Sil,
    YasDeyis,
    Cixis,
}
fn secimi_oxu(metn: &str) ->Option<Secim> {
    match metn{
        "1" =>Some(Secim::Siyahi),
        "2" =>Some(Secim::Elave),
        "3" =>Some(Secim::Saxla),
        "4" =>Some(Secim::Axtar),
        "5" =>Some(Secim::Sil),
        "6" =>Some(Secim::YasDeyis),
        "7" =>Some(Secim::Cixis),
        _=> None,
    }
}
fn main() {

    let mut telebeler = fayildan_oxu();

    loop{
        println!("\n1 - Telebelerin siyahisi:");
        println!("2 - Telebe elave et");
        println!("3 - Fayla saxla");
        println!("4 - Telebe axtar");
        println!("5 - Telebe sil");
        println!("6 - Yasi deyis");
        println!("7 - Cixis");

        let giris = oxu("Secim edin:");

        match secimi_oxu(&giris){
            Some(Secim::Siyahi) =>{
                for t in &telebeler{
                    println!("{}", t.tanit());
                }
            }
            Some(Secim::Elave) => {
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
            Some(Secim::Saxla) =>{
                match fayla_yaz(&telebeler){
                    Ok(_) =>println!("Saxlanlidi"),
                    Err(e) =>println!("Xeta: {e}"),
                }
            }
            Some(Secim::Axtar) =>{
                let ad = oxu("Axtarilan ad:");

                match tap(&telebeler, &ad) {
                    Some(t) =>println!("Tapildi: {}",t.tanit()),
                    None =>println!("Bele telebe yoxdur"),
                }
            }
            Some(Secim::Sil) =>{

                let ad = oxu("Silinecek ad:");

                match telebeler.iter().position(|t| t.ad == ad){
                    Some(yer) =>{
                        telebeler.remove(yer);
                        println!("Silindi");
                    }
                    None =>println!("Bele telebe yoxdur.")
                }
            }
            Some(Secim::YasDeyis) =>{
                let ad = oxu("Telebenin adi:");
                let yeni_yas = oxu("Yeni yas:");

                match yeni_yas.parse::<i32>(){
                    Ok(yas) =>match tap_deyis(&mut telebeler, &ad){
                        Some(t) =>{
                            t.yas = yas;
                            println!("Yenilendi");
                        }
                        None =>println!("Bele telebe yoxdur."),
                    },
                    Err(_) =>println!("Yas ededi duz deyil."),
                }
            }
            Some(Secim::Cixis) =>{
                println!("Gorusenedek!");
                break;
            }
            None =>println!("Bele secim yoxdur."),
        }
    }
}