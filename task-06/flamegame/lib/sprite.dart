import 'package:flame/components.dart';
import 'directions.dart';


    class Bunny extends SpriteComponent with HasGameRef {
      Bunny() : super(size: Vector2.all(100.0));
    
      @override
      Future<void> onLoad() async {
        super.onLoad();
        sprite = await gameRef.loadSprite('idle.png');
        position = gameRef.size / 2;
      }
      Direction direction = Direction.none;
    
    @override
    void update(double dt) {
      super.update(dt);
      updatePosition(dt);
    }
    
    updatePosition(double dt) {
      switch (direction) {
        case Direction.up:
          position.y=y-5;
          break;
        case Direction.down:
          position.y=y+5;
          break;
        case Direction.left:
          position.x=x-5;
          break;
        case Direction.right:
          position.x=x+5;
          break;
        case Direction.none:
          break;
      }
    }
    }