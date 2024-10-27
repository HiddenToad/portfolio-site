let result;

function round_to(n, places){
    exp = Math.pow(10, places);
    return Math.round(n * exp) / exp
}

function submit(){
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/train");
    xhr.onload = () => {
        if(xhr.responseText == "INVALIDINPUT"){
            alert("Whoops! That data wasn't formatted right.");
        }
        else{
            result = JSON.parse(xhr.responseText);
            alert("Training complete!");
        }
    }
    if (document.getElementById("data").value == ""){
        alert("Type something in first! The model can't learn from nothing :P");
        return;
    }
    xhr.send("data="+document.getElementById("data").value);
    
}

function predict(){
    if(result == undefined){
        alert("Training not yet complete.");
        return;
    }
    let value = document.getElementById("input").value;
    let prediction = result.intercept + result.slope * value;
    if(1.0 - (prediction % 1) <= 0.01){
        prediction = Math.round(prediction);
    }
    prediction = Math.round(prediction * 1000) / 1000;

    if(Number.isNaN(prediction)){
        document.getElementById("result").innerText = "Error: prediction input not in correct format.";
    }
    else{
        document.getElementById("result").innerText = "Model says: " + prediction;
        document.getElementById("formula").innerText = `Line of best fit: y = ${round_to(result.slope, 5)}x + ${round_to(result.intercept, 5)}`
        window.scrollTo(0, document.body.scrollHeight);
    }
}