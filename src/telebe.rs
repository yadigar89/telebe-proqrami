use std::{fs, io};
pub struct Telebe{
    pub ad: String,
    pub yas: i32,
    pub ixtisas: String,
}
pub trait Tanit{
    fn tanit(&self) ->String;
}
impl Tanit for Telebe{
    fn tanit(&self) ->String {
        format!("Ad: {}, yas: {}, ixtisas: {}",self.ad, self.yas, self.ixtisas)
    }
}
pub fn setirden_telebe(setir: &str) ->Result<Telebe, String>{
   
    let hisseler: Vec<&str> = setir.split(',').collect();

    if hisseler.len() != 3{
        return Err(String::from("Setirde 3 hisse olmalidir."));
    }
    let yas = hisseler[1]
       .trim()
       .parse::<i32>()
       .map_err(|_| String::from("Yas ededi duz deyil."))?;

    Ok(Telebe{
        ad: String::from(hisseler[0].trim()),
        yas,
        ixtisas: String::from(hisseler[2].trim()),
    })
}
pub fn fayildan_oxu() ->Vec<Telebe>{

    let mut telebeler: Vec<Telebe> = Vec::new();

    match fs::read_to_string("telebeler.txt"){
        Ok(metn) =>{
            for setir in metn.lines(){
                match setirden_telebe(setir){
                    Ok(t) =>telebeler.push(t),
                    Err(e) =>println!("Xeta: {e} ->{setir}"),
                }
            }
        }
        Err(_) =>println!("Fayl tapimadi bos siyahi ile baslayiriq."),
    }
    telebeler
}
pub fn fayla_yaz(telebeler: &[Telebe]) ->Result<(), std::io::Error>{

    let mut metn = String::new();

    for t in telebeler{
        metn.push_str(&format!("{}, {}, {}\n",t.ad, t.yas, t.ixtisas));
    }
    fs::write("telebeler.txt", metn)
}
pub fn oxu(izah: &str) -> String{

    println!("{izah}");

    let mut giris = String::new();

    match io::stdin().read_line(&mut giris){
        Ok(_) =>giris.trim().to_string(),
        Err(_) =>String::new(),
    }
}
pub fn tap<'a>(telebeler: &'a[Telebe], ad: &str) ->Option<&'a Telebe>{
    
    telebeler.iter().find(|t| t.ad == ad)
}
pub fn tap_deyis<'a>(telebeler: &'a mut [Telebe], ad: &str) ->Option<&'a mut Telebe>{

    telebeler.iter_mut().find(|t| t.ad == ad)
}
#[cfg(test)]
mod tesler {
    use super::*;

    #[test]
    fn duzgun_setir(){
        let netice = setirden_telebe("Yadigar, 36, Proqramci");
        assert!(netice.is_ok());
    }
    #[test]
    fn sehv_yas() {
        let netice = setirden_telebe("Yadigar, abc, Proqramci");
        assert!(netice.is_err());
    }
    #[test]
    fn az_hisse() {
        let netice = setirden_telebe("Yadigar, Proqramci");
        assert!(netice.is_err());
    }
    #[test]
        fn bosluq_temizlenir() {
            let netice = setirden_telebe(" Yadigar, 36, Proqramci ");
            let t = netice.unwrap();
            assert_eq!(t.ad, "Yadigar");
            assert_eq!(t.yas, 36);
            assert_eq!(t.ixtisas, "Proqramci");
        
    }

}