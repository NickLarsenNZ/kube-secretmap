# See: https://github.com/kube-rs/controller-rs/blob/main/Tiltfile
# See: https://github.com/tilt-dev/tilt-avatars/blob/main/Tiltfile

# Usage default features:
# tilt up
#
# Usage with features:
# tilt up telemetry
config.define_string("features", args=True)
cfg = config.parse()
features = cfg.get('features', "")
print("compiling with features: {}".format(features))

local_resource('compile', 'just compile %s' % features, deps=['src/'])
docker_build('secretmap/controller', '.', dockerfile='Dockerfile')
k8s_yaml('manifests/Deployment.yml')
k8s_resource('secretmap-controller', port_forwards=8080)
