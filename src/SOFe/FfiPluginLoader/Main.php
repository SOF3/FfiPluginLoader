<?php

namespace SOFe\FfiPluginLoader;

use pocketmine\plugin\{PluginBase, PluginLoadOrder};

final class Main extends PluginBase {
	public function onEnable() {
		$this->getServer()->getPluginManager()->registerInterface(new FfiPluginLoader);
		$this->getServer()->getPluginManager()->loadPlugins($this->getServer()->getPluginPath(), [FfiPluginLoader::class]);
		$this->getServer()->enablePlugins(PluginLoadOrder::STARTUP());
	}
}
