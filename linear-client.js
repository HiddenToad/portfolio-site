let result;
function submit(){
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/train");
    xhr.onload = () => {
        if(xhr.responseText == "INVALIDINPUT"){
            alert("Training failed: invalid input format.")
        }
        else{
            result = JSON.parse(xhr.responseText);
            alert("Training complete!");
        }
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
        document.getElementById("result").innerText = prediction;
    }
}