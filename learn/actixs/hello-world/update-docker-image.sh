#if you have wsl, just run bash first to activate linux subsystem and run chmod +x update-deployment.sh
# then you can run this script with ./update-deployment.sh
# update the names like actixs-app, jairjosafath/actixs-app to your own
# build docker image
docker build -t actixs-app .
# tag and push to docker hub
docker tag actixs-app:latest jairjosafath/actixs-app:latest
docker push jairjosafath/actixs-app:latest