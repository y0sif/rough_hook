/****************************************************************************
** Meta object code from reading C++ file 'mainwindow.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/mainwindow.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'mainwindow.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_MainWindow_t {
    QByteArrayData data[36];
    char stringdata0[462];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_MainWindow_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_MainWindow_t qt_meta_stringdata_MainWindow = {
    {
QT_MOC_LITERAL(0, 0, 10), // "MainWindow"
QT_MOC_LITERAL(1, 11, 7), // "addGame"
QT_MOC_LITERAL(2, 19, 0), // ""
QT_MOC_LITERAL(3, 20, 10), // "ChessGame*"
QT_MOC_LITERAL(4, 31, 4), // "game"
QT_MOC_LITERAL(5, 36, 18), // "openHookLensWindow"
QT_MOC_LITERAL(6, 55, 20), // "openRoughGuardWindow"
QT_MOC_LITERAL(7, 76, 7), // "newGame"
QT_MOC_LITERAL(8, 84, 13), // "newTournament"
QT_MOC_LITERAL(9, 98, 23), // "onWindowMenuAboutToShow"
QT_MOC_LITERAL(10, 122, 14), // "showGameWindow"
QT_MOC_LITERAL(11, 137, 17), // "updateWindowTitle"
QT_MOC_LITERAL(12, 155, 11), // "updateMenus"
QT_MOC_LITERAL(13, 167, 4), // "save"
QT_MOC_LITERAL(14, 172, 6), // "saveAs"
QT_MOC_LITERAL(15, 179, 12), // "onTabChanged"
QT_MOC_LITERAL(16, 192, 5), // "index"
QT_MOC_LITERAL(17, 198, 19), // "onTabCloseRequested"
QT_MOC_LITERAL(18, 218, 8), // "closeTab"
QT_MOC_LITERAL(19, 227, 11), // "destroyGame"
QT_MOC_LITERAL(20, 239, 20), // "onTournamentFinished"
QT_MOC_LITERAL(21, 260, 21), // "onGameManagerFinished"
QT_MOC_LITERAL(22, 282, 17), // "onGameStartFailed"
QT_MOC_LITERAL(23, 300, 14), // "onGameFinished"
QT_MOC_LITERAL(24, 315, 15), // "editMoveComment"
QT_MOC_LITERAL(25, 331, 3), // "ply"
QT_MOC_LITERAL(26, 335, 7), // "comment"
QT_MOC_LITERAL(27, 343, 7), // "copyFen"
QT_MOC_LITERAL(28, 351, 8), // "pasteFen"
QT_MOC_LITERAL(29, 360, 7), // "copyPgn"
QT_MOC_LITERAL(30, 368, 15), // "showAboutDialog"
QT_MOC_LITERAL(31, 384, 13), // "closeAllGames"
QT_MOC_LITERAL(32, 398, 14), // "adjudicateDraw"
QT_MOC_LITERAL(33, 413, 18), // "adjudicateWhiteWin"
QT_MOC_LITERAL(34, 432, 18), // "adjudicateBlackWin"
QT_MOC_LITERAL(35, 451, 10) // "resignGame"

    },
    "MainWindow\0addGame\0\0ChessGame*\0game\0"
    "openHookLensWindow\0openRoughGuardWindow\0"
    "newGame\0newTournament\0onWindowMenuAboutToShow\0"
    "showGameWindow\0updateWindowTitle\0"
    "updateMenus\0save\0saveAs\0onTabChanged\0"
    "index\0onTabCloseRequested\0closeTab\0"
    "destroyGame\0onTournamentFinished\0"
    "onGameManagerFinished\0onGameStartFailed\0"
    "onGameFinished\0editMoveComment\0ply\0"
    "comment\0copyFen\0pasteFen\0copyPgn\0"
    "showAboutDialog\0closeAllGames\0"
    "adjudicateDraw\0adjudicateWhiteWin\0"
    "adjudicateBlackWin\0resignGame"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_MainWindow[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      29,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    1,  159,    2, 0x0a /* Public */,
       5,    0,  162,    2, 0x08 /* Private */,
       6,    0,  163,    2, 0x08 /* Private */,
       7,    0,  164,    2, 0x08 /* Private */,
       8,    0,  165,    2, 0x08 /* Private */,
       9,    0,  166,    2, 0x08 /* Private */,
      10,    0,  167,    2, 0x08 /* Private */,
      11,    0,  168,    2, 0x08 /* Private */,
      12,    0,  169,    2, 0x08 /* Private */,
      13,    0,  170,    2, 0x08 /* Private */,
      14,    0,  171,    2, 0x08 /* Private */,
      15,    1,  172,    2, 0x08 /* Private */,
      17,    1,  175,    2, 0x08 /* Private */,
      18,    1,  178,    2, 0x08 /* Private */,
      19,    1,  181,    2, 0x08 /* Private */,
      20,    0,  184,    2, 0x08 /* Private */,
      21,    0,  185,    2, 0x08 /* Private */,
      22,    1,  186,    2, 0x08 /* Private */,
      23,    1,  189,    2, 0x08 /* Private */,
      24,    2,  192,    2, 0x08 /* Private */,
      27,    0,  197,    2, 0x08 /* Private */,
      28,    0,  198,    2, 0x08 /* Private */,
      29,    0,  199,    2, 0x08 /* Private */,
      30,    0,  200,    2, 0x08 /* Private */,
      31,    0,  201,    2, 0x08 /* Private */,
      32,    0,  202,    2, 0x08 /* Private */,
      33,    0,  203,    2, 0x08 /* Private */,
      34,    0,  204,    2, 0x08 /* Private */,
      35,    0,  205,    2, 0x08 /* Private */,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Bool,
    QMetaType::Bool,
    QMetaType::Void, QMetaType::Int,   16,
    QMetaType::Void, QMetaType::Int,   16,
    QMetaType::Void, QMetaType::Int,   16,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, QMetaType::Int, QMetaType::QString,   25,   26,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void MainWindow::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<MainWindow *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->addGame((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 1: _t->openHookLensWindow(); break;
        case 2: _t->openRoughGuardWindow(); break;
        case 3: _t->newGame(); break;
        case 4: _t->newTournament(); break;
        case 5: _t->onWindowMenuAboutToShow(); break;
        case 6: _t->showGameWindow(); break;
        case 7: _t->updateWindowTitle(); break;
        case 8: _t->updateMenus(); break;
        case 9: { bool _r = _t->save();
            if (_a[0]) *reinterpret_cast< bool*>(_a[0]) = std::move(_r); }  break;
        case 10: { bool _r = _t->saveAs();
            if (_a[0]) *reinterpret_cast< bool*>(_a[0]) = std::move(_r); }  break;
        case 11: _t->onTabChanged((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 12: _t->onTabCloseRequested((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 13: _t->closeTab((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 14: _t->destroyGame((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 15: _t->onTournamentFinished(); break;
        case 16: _t->onGameManagerFinished(); break;
        case 17: _t->onGameStartFailed((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 18: _t->onGameFinished((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 19: _t->editMoveComment((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< const QString(*)>(_a[2]))); break;
        case 20: _t->copyFen(); break;
        case 21: _t->pasteFen(); break;
        case 22: _t->copyPgn(); break;
        case 23: _t->showAboutDialog(); break;
        case 24: _t->closeAllGames(); break;
        case 25: _t->adjudicateDraw(); break;
        case 26: _t->adjudicateWhiteWin(); break;
        case 27: _t->adjudicateBlackWin(); break;
        case 28: _t->resignGame(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject MainWindow::staticMetaObject = { {
    QMetaObject::SuperData::link<QMainWindow::staticMetaObject>(),
    qt_meta_stringdata_MainWindow.data,
    qt_meta_data_MainWindow,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *MainWindow::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *MainWindow::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_MainWindow.stringdata0))
        return static_cast<void*>(this);
    return QMainWindow::qt_metacast(_clname);
}

int MainWindow::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QMainWindow::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 29)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 29;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 29)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 29;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
