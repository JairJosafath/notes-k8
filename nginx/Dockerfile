# Use the official Nginx image as the base
FROM nginx:latest

# Copy custom nginx.conf to the container
COPY nginx.conf /etc/nginx/nginx.conf

# Copy static files to the container
COPY index.html /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start Nginx nginx -s signal
CMD ["nginx", "-g", "daemon off;"]
