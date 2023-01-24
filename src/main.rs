*{
    margin: 0;
    font-family: "ONE Mobile POP OTF"
}
body{
    background-color: aliceblue;
}
@font-face {
    font-family: 'ONE Mobile POP OTF';
    src: url('https://cdn.jsdelivr.net/gh/wooin21/web/fonts/onestore/ONE Mobile POP OTF.woff') format('woff');
    font-weight: normal;
    font-style: normal;
}

.Com > #count{
    padding-top: 50px;
    margin-bottom: 10px;
    color: rgb(0, 136, 255);
    font-size: 50px;
    text-align: center;
    transform-style: flat;
}

.Com > #btn{
    display: block;
    margin: 0 auto;
    border: solid 0px;
    color: #fff;
    background-color: rgb(0, 136, 255);
    border-radius: 20px;
    font-size: 50px;
    padding: 10px 200px;
    cursor: pointer;
}

.Com > #btn:hover{
    background-color: rgb(0, 123, 231);
}

.boom1{
    animation-duration: .1s;
    animation-name: clicked1;
}
.boom2{
    animation-duration: .1s;
    animation-name: clicked2;
}

@keyframes clicked1{0%{transform:rotate(0deg); font-size: 70; }to{transform:rotate(1turn)}}
@keyframes clicked2{0%{transform:rotate(0deg); font-size: 70; }to{transform:rotate(1turn)}}


::-webkit-scrollbar{
    display:none
}
