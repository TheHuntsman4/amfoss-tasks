import 'dart:ui';
import 'directions.dart';
    
    import 'package:flame/game.dart';
    import 'sprite.dart';
    import 'background.dart';
    
    class BunnyGame extends FlameGame{
      Bunny _bunny = Bunny();
    Background _world = Background();
      @override
      Future<void> onLoad() async {
        super.onLoad();
        await add(_world);
        await add(_bunny);
        _bunny.position = Vector2(0, 900);
        camera.followComponent(_bunny,
            worldBounds: Rect.fromLTRB(0, 0, _world.size.x, _world.size.y));
      }
      onArrowKeyChanged(Direction direction){
      _bunny.direction = direction;
    }
    }