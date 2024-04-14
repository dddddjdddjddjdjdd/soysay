use std::io::{stdin, Read};

use clap::{Arg, Command};
use rand::{thread_rng, prelude::SliceRandom};

struct Soyjak {
    id: &'static str,
    name: &'static str,
    art: &'static str
}

static SOYJAKS: [Soyjak; 6] = [
    Soyjak {
        id: "soyjak",
        name: "Soyjak",
        art: "     \\                                     
      \\     ...-############-...           
       \\   -++--.            .--++-        
       ..+-          ...         -+...      
     -+-.           .-.          .--+-      
    ++             ......            -#.    
    ++             .-----.-.         .-+.   
    ++                                .#.   
   -+-         .---------    .--------  #-  
   #-         -+.........    ........-+.#-  
   #-                      ..            ++ 
   #-           .+##########+            .-#
   #--+   .++++++#- .#####.++----+#####+##+ 
   #--+         .#- .#####.++....-#####.#-+ 
   #-.          .#-        ++    -####+.+-+ 
    ++          .##++++++++#+  +--#++++++#+ 
    ++.   ..#-                  -+..    +-  
     .-+. .-+.                   -#.        
      .#.  .#-.         -#. ...  -#.    ##+ 
      .#.    ++             .--.         #- 
      .#.  .##+          .#-|||##+     #-   
      .#.    -++-.      -#+#####++#. .-+.   
      .#.      ..#-     -##########. -#.    
      .#.     --.#-     -########+  .--.    
     -+..       .#-     ..######+..+-.      
    .-+   -#.     -####+--+##+-. .-+.       
   #-.   -+..      ...#+.-+######+..        "
    },
    Soyjak {
        id: "snoo",
        name: "Snoojak",
        art: "     \\                                     
      \\                *@*-      +@@@@+    
       \\               @@ -@@@@@@@    @|   
        \\             @@        @@    @|   
         \\           =@*         +@@@@+    
          \\          @@                    
                   *@%@%+                   
           -@@@@@@+:....:+@@@@@@            
 +@@@@@@@@@+                    #@@%@@@@@@+ 
@@...:@@.                          -@@    @@
@@  @@        .:.           ..       .@@  @@
*@-@@       :*@@@*        *@@@*:       @@-@*
  @@        +*==**        **==*+        @@  
  @@                                    @#  
  @@ -        #     %@@=      +        :@-  
   @@:=+     . =   + =  * +:-@ -= =-.  @%   
    %@+.-*    *@@@@@@@@@@@@@@#.   = +%@@    
      @@@#:.-  @@@@@@@@@@@@@+ : .:=@@%      
        =@@@ :.=@@@@@@@@@@@.  .:#@@:        
            @#  :@@@@@@@@@@   =@#           
             %%-. @@- #.+@ . :@=+           
               @-*   =%*   .@:*             
               *%@= .:+*:+@+-               
                  - *#+*+*                  "
    },
    Soyjak {
        id: "gape",
        name: "Gapejak",
        art: "     \\                                     
      \\    +################+              
       \\  +                  #             
         .                       #          
        #           #     -#                
       .    .     .                         
       -             #.   .###       #      
   .###     #   -#             ##     #     
 #   +        #  #     ####   .  ##    #    
##       ###     ##  #  ###+- #    #        
 #  #           +# +#####+           +  +   
  # #    #     ##   #                 #     
   #    # ###   ##  -                  +    
  #       ..      #                      +  
                                          # 
 #         .####-                         # 
         # # # # #  #                     # 
#         ##########  -                   # 
#         #############-  .         #     # 
         ###############   #       .+-   #  
 .-      ###############           # .#- ## 
  #      ################   .         ##-+. 
 .#      ################    +     # ####   
  ##   .  ###############            # ##-  
   #+      #############              +#    
   ##.     #############             ###.   
    +#     -##########-#           # ####   
    ##      # ######.##       #   ## ##     
    + #-      #.#  ##-      .###  #+#- +    
     ###  ##              ####   ####     # 
   + ####+ # #           # -- ####-+## -    
#    .# ##   ##        # ##  ####    #      
         .###.## -######## # ###-           
        #####      #--   # -# +             
           .-##+     +# ### #.#             
              ##+###++####    #             
              . ##   # #                    "
    },
    Soyjak {
        id: "cobson",
        name: "Cobson soyjak",
        art: "    /                                       
   /    .                                   
  /   .                          #          
    .                             .         
                                            
   .                                        
                                    #       
                         #####              
              #       .######### ----#      
  #               ###### -- .- ##           
  +#########-     #-     #-.#   #     -     
  ##  # --             +##             +    
###   ##.+ ##      #                        
  -   #-            #  -#-              -   
 .   -##--#                                 
  #                                         
  -       +              .               #  
          -       ####    #              #  
           ##+    ##.      +             ## 
            #                            ## 
  #-                                     -# 
  ##      #                              +##
  .-#         +         -                -# 
    #-         #  +                      # #
   ###       #. +     #####-            ##  
    ##       ################     +    ##-  
     ##      ##################        ##-- 
 #   -##      #################    #   ##   
      -##      ###############         #-   
      ###      ###############       ##--   
       ###      ##-########          -#     
         #.      # ###### ##        -##     
         ###      .#  + .#    #-    #       
           .#  ##           ###    #.       
            +#+ + # -#-# # +       #+       
             #-       +         ## #        
              ###+             ###          "
    },
    Soyjak {
        id: "markiplier",
        name: "Markiplier soyjak",
        art: "    \\                                      
     \\        #################            
      \\     #+                 +#          
          ###                     ###       
         #                           #      
       ##                             #     
      #           ##   ##              #    
     #      #                          #    
     #     #.    #          -#  ##     #    
     #            #        -           +    
     ####  ##########      .####   #######  
    ###     #   #+  #    #     ###     ###  
     #    -#  ###  ##    #+#  ##  #    ##   
     ##             #     #            #+   
     ### ###########      +#####       #    
     #                                 #    
     #            #         #          #    
     #                        #        #    
    -           # ++       ###          -   
     #                                   ###
    #          #  ######+     #          #  
   #+        #  .##  #  # # ##    #     ### 
    +      +   #  #  #  # # # #   #     .## 
    ## #   -  ##################  . # .+# # 
    ### +  #  ##################    # + #   
     #.-+#-   ##################   #   #    
     ## # #   ##################    ## ##   
   ### #####  ##################  #   #.#   
     ##.-  #  ##################  . ###- ## 
     - ##  +  ##################   ##-   #  
      ## -  .  ################  +  ## #+#  
       #+.# .   ##############   #.# # +#   
          ##.     ###########    +#  +#     
          ###      #######+      +     .    
           ######        .     .+##         
            ## #-   # # #    #####          
              ####   # +  ### ###           
                 .## ###..+ ###             "
    },
    Soyjak {
        id: "chud",
        name: "Chudjak",
        art: "     \\                                     
      \\            ##########              
       \\       #####################       
        \\   ###########################    
            #################      ++ ###   
           #########                    #   
           #######.                      #  
           #######                        # 
           ######                 .       # 
            #####            #    #       # 
             ###    -----.   .    .   .---  
             ##     ### #####     # --####  
            #       #         ## ###      # 
           #        # - ###.       # ###  # 
           #        ##  ++   #     #  #   # 
           #                +      +######  
           #                              + 
           #               #+#      -    +  
           #               .# ##  ##        
           #              #            #    
           #               #    +           
           #                #     # #.      
                               +    #       
          #                        #        
         #     +               ##           
       #     .             +##  ##          "
    }
];

fn main() {
    let mut cmd = Command::new("soysay")
        .arg(Arg::new("random")
            .required(false)
            .num_args(0)
            .help("Picks a random soyjak to display")
            .long("random")
            .short('r'))
            .after_help("Note: If no options are provided, soysay will use the standard soyjak.");
    for soyjak in SOYJAKS.iter().skip(1) {
        let arg = Arg::new(soyjak.id)
            .required(false)
            .long(soyjak.id)
            .num_args(0)
            .help(format!("Uses the {}", soyjak.name));
        cmd = cmd.arg(arg);
    }
    cmd = cmd.arg(Arg::new("input")
        .required(false)
        .num_args(1..)
        .trailing_var_arg(true));
    let matches = cmd.get_matches();
    let input = if matches.contains_id("input") {
        matches.get_many::<String>("input")
            .unwrap()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    } else {
        let mut buf = Vec::new();
        stdin().lock()
            .read_to_end(&mut buf)
            .unwrap();
        String::from_utf8_lossy(&buf)
            .to_string()
    };
    let input = input.trim();
    if input.is_empty() {
        return;
    }
    let soyjak = if matches.get_flag("random") {
        SOYJAKS.choose(&mut thread_rng()).unwrap()
    } else {
        SOYJAKS.iter()
            .skip(1)
            .filter(|s| matches.get_flag(s.id))
            .next()
            .unwrap_or(&SOYJAKS[0])
    };
    let line_count = input.lines().count();
    let width = input.lines().fold(0, |a,b| a.max(b.len()));
    println!("  {} ", str::repeat("_", width + 2));
    if line_count == 1 {
        println!(" < {} >", input);
    } else {
        for (i, line) in input.lines().enumerate() {
            if i == 0 {
                print!(" / {}", line);
            } else if i == line_count - 1 {
                print!(" \\ {}", line);
            } else {
                print!(" | {}", line);
            }
            for _ in 0..(width - line.len()) {
                print!(" ");
            }
            if i == 0 {
                println!(" \\");
            } else if i == line_count - 1 {
                println!(" /");
            } else {
                println!(" |");
            }
        }
    }
    println!("  {} ", str::repeat("-", width + 2));
    println!("{}", soyjak.art);
}
