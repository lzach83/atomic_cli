pub struct Logo {
    pub logo: String,
}

impl Logo {
    pub fn show() -> &'static str {
        let logo: &str = "

                                                                                                                     
                                                                                                                     
               AAA                tttt                                                     iiii                      
              A:::A            ttt:::t                                                    i::::i                     
             A:::::A           t:::::t                                                     iiii                      
            A:::::::A          t:::::t                                                                               
           A:::::::::A   ttttttt:::::ttttttt       ooooooooooo      mmmmmmm    mmmmmmm   iiiiiii     cccccccccccccccc
          A:::::A:::::A  t:::::::::::::::::t     oo:::::::::::oo  mm:::::::m  m:::::::mm i:::::i   cc:::::::::::::::c
         A:::::A A:::::A t:::::::::::::::::t    o:::::::::::::::om::::::::::mm::::::::::m i::::i  c:::::::::::::::::c
        A:::::A   A:::::Atttttt:::::::tttttt    o:::::ooooo:::::om::::::::::::::::::::::m i::::i c:::::::cccccc:::::c
       A:::::A     A:::::A     t:::::t          o::::o     o::::om:::::mmm::::::mmm:::::m i::::i c::::::c     ccccccc
      A:::::AAAAAAAAA:::::A    t:::::t          o::::o     o::::om::::m   m::::m   m::::m i::::i c:::::c             
     A:::::::::::::::::::::A   t:::::t          o::::o     o::::om::::m   m::::m   m::::m i::::i c:::::c             
    A:::::AAAAAAAAAAAAA:::::A  t:::::t    tttttto::::o     o::::om::::m   m::::m   m::::m i::::i c::::::c     ccccccc
   A:::::A             A:::::A t::::::tttt:::::to:::::ooooo:::::om::::m   m::::m   m::::mi::::::ic:::::::cccccc:::::c
  A:::::A               A:::::Att::::::::::::::to:::::::::::::::om::::m   m::::m   m::::mi::::::i c:::::::::::::::::c
 A:::::A                 A:::::A tt:::::::::::tt oo:::::::::::oo m::::m   m::::m   m::::mi::::::i  cc:::::::::::::::c
AAAAAAA                   AAAAAAA  ttttttttttt     ooooooooooo   mmmmmm   mmmmmm   mmmmmmiiiiiiii    cccccccccccccccc
                                                                                                                     
                                                                                                                     
                                                                                                                     
                                                                                                                     
                                                                                                                     
                                                                                                                     
                                                                                                                     
";
        logo
    }
}
