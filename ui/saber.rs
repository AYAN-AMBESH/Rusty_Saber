import { Button, GridLayout, LineEdit, Rectangle } from "std-widgets.slint";

export component Saber inherits Window{
    background:blue;
    preferred-width: 200px;
    preferred-height: 200px;
    callback  dir(string) -> string;
    GridLayout{
        Row{
        LineEdit{
            font-size: 14px;
            width: 50px;
            height: 20px;
            placeholder-text: "Enter directory name here";
            Button{
                text: "Click to scan";
                clicked => { root.dir(text)}
                }
            }
        
        }

    }
}