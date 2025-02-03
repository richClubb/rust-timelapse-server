# Rust Timelapse Server

This is a server that can correlate images sent from webcams to create timelapses

## Overview

* Endpoint to push files to
* JWT auth
* Adds to database / filestore to allow retrieval
* Retrieval of images as a zip or to a gif / mp4
* Timestamping
* Post-processing of images
    * Idea is to offload the processing from the endpoint to the main processor which will have more grunt to be able to postprocess the images
    * object detection
    * motion capture
    * facial recognition
    * delta from last image
    * intelligent motion / interest categorization
* Ability to add plugins

## MVP V0.1
 
* Endpoint to push files to
* Adds to database / filestore
* Timestamping


