# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/vivid64"

  config.vm.provision "shell", path: "vagrant/provision.sh", privileged: false
end
