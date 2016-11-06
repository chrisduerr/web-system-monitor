function get_data_async(theUrl) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            update(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true);
    xmlHttp.send(null);
}

function sleep(time) {
    return new Promise((resolve) => setTimeout(resolve, time));
}

function update(response) {
    var responseJSON = JSON.parse(response);
    var cpu_avg = Math.trunc(responseJSON["cpu"]["avg"]);
    document.getElementById("cpu_label").textContent = cpu_avg + "%";
    document.getElementById("cpu_path").setAttribute('d', getPathFromPercentage(cpu_avg));

    document.getElementById("testdiv").innerText = response;

    sleep(100).then(() => get_data_async("/get_raw_data"));
}

get_data_async("/get_raw_data");


// ----- GRAPH STUFF -----
var zeroPercPoint = [25, 85];
var hundredPercPoint = [75, 85];
var centerPoint = [50, getCenterYPos()];

var zeroPercVector = [-25, centerPoint[1] - zeroPercPoint[1]];
var hundredPercVector = [25, centerPoint[1] - hundredPercPoint[1]];

var angleRad = getRadAngleBetweenStartVectors();

// Formula used: http://mathforum.org/library/drmath/view/53027.html
function getCenterYPos() {
	var distanceBetweenPoints = 50;
  var radius = 40;
  var halfWayPoint = [50, 85];
  return halfWayPoint[1] - Math.sqrt(Math.pow(radius, 2) - Math.pow((distanceBetweenPoints / 2), 2)) * (hundredPercPoint[0] - zeroPercPoint[0]) / distanceBetweenPoints;
}

// Formula used: http://www.wikihow.com/Find-the-Angle-Between-Two-Vectors
function getRadAngleBetweenStartVectors() {
  var numerator = zeroPercVector[0] * hundredPercVector[0] + zeroPercVector[1] * hundredPercVector[1];
  var denominator = Math.sqrt(Math.pow(zeroPercVector[0], 2) + Math.pow(zeroPercVector[1], 2)) * Math.sqrt(Math.pow(hundredPercVector[0], 2) + Math.pow(hundredPercVector[1], 2));
  var cos = numerator / denominator;
  return 2 * Math.PI - Math.acos(cos);
}

function getRadAngleFromStartVector(percentage) {
	return angleRad * (percentage / 100);
}

function getVectorFromPercentage(percentage) {
  percentage = 100 - percentage;
  var radAngle = getRadAngleFromStartVector(percentage);
  var newX = zeroPercVector[0] * Math.cos(radAngle) + zeroPercVector[1] * Math.sin(radAngle);
  var newY = zeroPercVector[0] * Math.sin(radAngle) - zeroPercVector[1] * Math.cos(radAngle);
  newX += zeroPercVector[0];
  newY += zeroPercVector[1];
  newX *= -1;
  return [newX, newY];
}

// This is called from the outside to get the new path
function getPathFromPercentage(percentage) {
  var absolutePosition = getVectorFromPercentage(percentage);
  if (percentage <= 63) {
  	return "M25,85 a40,40 0 0,1 " + absolutePosition[0] + "," + absolutePosition[1];
  }
  return "M25,85 a40,40 0 1,1 " + absolutePosition[0] + "," + absolutePosition[1];
}
