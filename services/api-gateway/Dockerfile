# Multi-stage Java build
# Stage 1: maven:3.9-eclipse-temurin-21, mvn package -DskipTests
# Stage 2: eclipse-temurin:21-jre-slim, copy jar
# JVM flags: -XX:+UseZGC -Xmx2g --enable-preview
# EXPOSE 800x (see docker-compose.yml for port mapping)
