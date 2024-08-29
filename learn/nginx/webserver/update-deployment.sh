#if you have wsl, just run bash first to activate linux subsystem and run chmod +x update-deployment.sh
# then you can run this script with ./update-deployment.sh
# update the names like my-custom-nginx, jairjosafath/k8-notes to your own
# build docker image
docker build -t my-custom-nginx .
# tag and push to docker hub
docker tag my-custom-nginx:latest jairjosafath/k8-notes:latest
docker push jairjosafath/k8-notes:latest
#echo "Deployment updated"
echo "Deployment updated"