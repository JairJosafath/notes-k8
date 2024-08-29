#if you have wsl, just run bash first to activate linux subsystem and run chmod +x update-deployment.sh
# then you can run this script with ./update-deployment.sh
# update the names like notes-nginx-webserver, jairjosafath/notes-nginx-webserver to your own
# build docker image
docker build -t notes-nginx-webserver .
# tag and push to docker hub
docker tag notes-nginx-webserver:latest jairjosafath/notes-nginx-webserver:latest
docker push jairjosafath/notes-nginx-webserver:latest